//! Transformer Inference Engine
//!
//! Minimal transformer implementation for running LLM inference.
//! Supports the LLaMA architecture (used by most open models).
//!
//! This runs IN the kernel — no OS userspace needed.

extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;

/// Model configuration (extracted from GGUF metadata)
#[derive(Debug, Clone)]
pub struct ModelConfig {
    pub architecture: String,
    pub vocab_size: u32,
    pub hidden_size: u32,       // aka embedding_length
    pub num_layers: u32,        // aka block_count
    pub num_heads: u32,         // aka attention.head_count
    pub num_kv_heads: u32,      // aka attention.head_count_kv (for GQA)
    pub intermediate_size: u32, // aka feed_forward_length
    pub max_seq_len: u32,       // aka context_length
    pub head_dim: u32,          // hidden_size / num_heads
    pub rope_theta: f32,       // RoPE base frequency
}

impl ModelConfig {
    /// Extract config from GGUF metadata
    pub fn from_gguf(gguf: &crate::gguf::GgufFile) -> Option<Self> {
        let arch_str = gguf.architecture()?; let arch = alloc::string::String::from(arch_str);
        let prefix = alloc::string::String::from("info");

        let get_u32 = |suffix: &str| -> Option<u32> {
            let key = alloc::string::String::from("info");
            gguf.get_u32(&key)
        };

        let hidden = get_u32("embedding_length")?;
        let heads = get_u32("attention.head_count")?;

        Some(ModelConfig {
            architecture: arch,
            vocab_size: get_u32("vocab_size").unwrap_or(32000),
            hidden_size: hidden,
            num_layers: get_u32("block_count")?,
            num_heads: heads,
            num_kv_heads: get_u32("attention.head_count_kv").unwrap_or(heads),
            intermediate_size: get_u32("feed_forward_length").unwrap_or(hidden * 4),
            max_seq_len: get_u32("context_length").unwrap_or(2048),
            head_dim: hidden / heads,
            rope_theta: 10000.0, // Default, can be overridden
        }
    }
}

/// Runtime state for inference
pub struct InferenceState {
    pub config: ModelConfig,
    pub kv_cache: Vec<KVCacheLayer>,
    pub position: u32,
    pub temperature: f32,
    pub top_p: f32,
    pub max_tokens: u32,
}

/// Key-Value cache for one transformer layer
pub struct KVCacheLayer {
    pub key: Vec<f32>,   // [seq_len, num_kv_heads, head_dim]
    pub value: Vec<f32>, // [seq_len, num_kv_heads, head_dim]
}

impl InferenceState {
    pub fn new(config: ModelConfig) -> Self {
        let num_layers = config.num_layers as usize;
        let mut kv_cache = Vec::with_capacity(num_layers);
        for _ in 0..num_layers {
            kv_cache.push(KVCacheLayer {
                key: Vec::new(),
                value: Vec::new(),
            });
        }

        InferenceState {
            config,
            kv_cache,
            position: 0,
            temperature: 0.7,
            top_p: 0.9,
            max_tokens: 256,
        }
    }
}

// ============================================================
// Math operations for transformer inference
// ============================================================

/// Matrix multiplication: C = A * B
/// A: [m, k], B: [k, n] → C: [m, n]
pub fn matmul(a: &[f32], b: &[f32], m: usize, k: usize, n: usize) -> Vec<f32> {
    let mut c = alloc::vec![0.0f32; m * n];
    for i in 0..m {
        for j in 0..n {
            let mut sum = 0.0f32;
            for p in 0..k {
                sum += a[i * k + p] * b[p * n + j];
            }
            c[i * n + j] = sum;
        }
    }
    c
}

/// RMSNorm: x = x * rsqrt(mean(x^2) + eps) * weight
pub fn rms_norm(x: &mut [f32], weight: &[f32], eps: f32) {
    let n = x.len();
    let mut sum_sq = 0.0f32;
    for &v in x.iter() {
        sum_sq += v * v;
    }
    let scale = 1.0 / crate::math::sqrtf(sum_sq / n as f32 + eps);
    for i in 0..n {
        x[i] = x[i] * scale * weight[i];
    }
}

/// Softmax in-place
pub fn softmax(x: &mut [f32]) {
    let max = x.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let mut sum = 0.0f32;
    for v in x.iter_mut() {
        *v = crate::math::expf(*v - max);
        sum += *v;
    }
    for v in x.iter_mut() {
        *v /= sum;
    }
}

/// SiLU activation: x * sigmoid(x)
pub fn silu(x: f32) -> f32 {
    x / (1.0 + crate::math::expf(-x))
}

/// Apply RoPE (Rotary Position Embedding)
pub fn apply_rope(q: &mut [f32], k: &mut [f32], head_dim: usize, position: u32, theta: f32) {
    for i in (0..head_dim).step_by(2) {
        let freq = 1.0 / crate::math::powf(theta, i as f32 / head_dim as f32);
        let angle = position as f32 * freq;
        let cos = crate::math::cosf(angle);
        let sin = crate::math::sinf(angle);

        // Rotate Q
        let q0 = q[i];
        let q1 = q[i + 1];
        q[i] = q0 * cos - q1 * sin;
        q[i + 1] = q0 * sin + q1 * cos;

        // Rotate K
        let k0 = k[i];
        let k1 = k[i + 1];
        k[i] = k0 * cos - k1 * sin;
        k[i + 1] = k0 * sin + k1 * cos;
    }
}

/// Sample from a probability distribution
pub fn sample_top_p(probs: &[f32], top_p: f32) -> u32 {
    // Simple top-p (nucleus) sampling
    let mut indexed: Vec<(usize, f32)> = probs.iter().cloned().enumerate().collect();
    indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(core::cmp::Ordering::Equal));

    let mut cumulative = 0.0f32;
    let mut candidates = Vec::new();
    for (idx, prob) in &indexed {
        cumulative += prob;
        candidates.push((*idx, *prob));
        if cumulative >= top_p {
            break;
        }
    }

    // Simple weighted random (using a basic PRNG since we don't have RDRAND here)
    // For now, just return the highest probability token (greedy)
    if let Some(r) = crate::crypto::rdrand64() {
        let r = (r as f64 / u64::MAX as f64) as f32;
        let mut cum = 0.0f32;
        // Renormalize
        let total: f32 = candidates.iter().map(|(_, p)| p).sum();
        for (idx, prob) in &candidates {
            cum += prob / total;
            if cum >= r {
                return *idx as u32;
            }
        }
    }

    // Fallback: greedy
    candidates.first().map(|(idx, _)| *idx as u32).unwrap_or(0)
}

/// The main inference loop — generate one token
/// (This is a skeleton — needs weight loading to actually work)
pub fn generate_token(
    _state: &mut InferenceState,
    _token_id: u32,
    _weights: &[u8],
) -> u32 {
    // TODO: Implement full forward pass
    // 1. Token embedding lookup
    // 2. For each layer:
    //    a. RMSNorm
    //    b. QKV projection (matmul)
    //    c. RoPE
    //    d. Attention (scaled dot product)
    //    e. RMSNorm  
    //    f. FFN (gate_proj, up_proj, down_proj with SiLU)
    // 3. Final RMSNorm
    // 4. LM head projection
    // 5. Softmax + sampling
    
    // For now, return EOS to indicate "not yet implemented"
    2 // EOS token
)}
