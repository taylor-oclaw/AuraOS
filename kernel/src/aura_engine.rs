//! Aura Engine — AI Model Runtime for AuraOS
//!
//! This is the core AI infrastructure of the OS.
//! It manages model loading, inference, and routing.
//!
//! Architecture:
//! 1. Model Registry — install/remove/update models
//! 2. Hardware Profiler — detect what the machine can run
//! 3. Model Router — pick the right model for each task
//! 4. Resource Governor — never starve human tasks
//! 5. Provider Manager — API keys for cloud models

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

/// The main Aura Engine state
pub struct AuraEngine {
    pub state: EngineState,
    pub local_models: Vec<ModelInfo>,
    pub cloud_providers: Vec<CloudProvider>,
    pub hardware_profile: HardwareProfile,
    pub resource_budget: ResourceBudget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineState {
    /// No models loaded — using pattern matching only
    PatternMatchOnly,
    /// Loading a local model into memory
    LoadingModel,
    /// A local model is ready for inference
    LocalReady,
    /// Using cloud API (local not available)
    CloudOnly,
    /// Both local and cloud available
    HybridReady,
}

/// Information about an installed model
#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub name: String,
    pub version: String,
    pub size_bytes: u64,
    pub parameter_count: u64,      // e.g., 3_800_000_000 for 3.8B
    pub quantization: Quantization,
    pub capabilities: Vec<ModelCapability>,
    pub ram_required_mb: u32,
    pub gpu_vram_required_mb: u32,
    pub tokens_per_second: f32,    // Benchmarked on this hardware
    pub loaded: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quantization {
    F32,     // Full precision (huge)
    F16,     // Half precision
    Q8_0,    // 8-bit quantization
    Q5_1,    // 5-bit quantization
    Q4_0,    // 4-bit quantization (smallest, fast)
    Q3_K_M,  // 3-bit quantization (tiny)
    Q2_K,    // 2-bit (experimental)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelCapability {
    Chat,           // Conversational
    CodeGeneration, // Write code
    CodeReview,     // Analyze code
    Summarization,  // Summarize text
    Translation,    // Language translation
    ImageCaption,   // Describe images (vision model)
    SpeechToText,   // Transcribe audio
    TextToSpeech,   // Generate speech
    Embedding,      // Vector embeddings
    ToolUse,        // Function calling
    Reasoning,      // Complex reasoning
    FastResponse,   // Quick, short responses
    OSControl,      // Understands OS actions
}

/// Cloud AI provider configuration
#[derive(Debug, Clone)]
pub struct CloudProvider {
    pub name: String,
    pub provider_type: ProviderType,
    pub auth: ProviderAuth,
    pub endpoint: String,
    pub enabled: bool,
    pub models: Vec<String>,
    pub cost_per_1k_tokens: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderType {
    Anthropic,     // Claude
    OpenAI,        // GPT, Codex
    Google,        // Gemini
    Perplexity,    // Perplexity
    Groq,          // Fast inference
    Ollama,        // Local Ollama server
    Custom,        // Any OpenAI-compatible API
}

#[derive(Debug, Clone)]
pub enum ProviderAuth {
    ApiKey(String),
    OAuth { token: String, refresh_token: String },
    None,
}

/// Hardware profile for model selection
#[derive(Debug, Clone)]
pub struct HardwareProfile {
    pub cpu_cores: u32,
    pub cpu_freq_mhz: u32,
    pub ram_total_mb: u64,
    pub ram_available_mb: u64,
    pub gpu_detected: bool,
    pub gpu_vram_mb: u32,
    pub gpu_compute_capability: Option<String>,
    pub disk_speed_mbps: u32,
    pub recommended_tier: ModelTier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelTier {
    /// < 4GB RAM: Only tiny models (1-3B)
    Tiny,
    /// 4-8GB RAM: Small models (3-7B)
    Small,
    /// 8-16GB RAM: Medium models (7-13B)
    Medium,
    /// 16-32GB RAM: Large models (13-32B)
    Large,
    /// 32GB+ RAM: Very large models (32-70B+)
    VeryLarge,
}

/// Resource budget for AI inference
#[derive(Debug, Clone)]
pub struct ResourceBudget {
    pub max_cpu_percent: u8,       // How much CPU the AI can use
    pub max_ram_mb: u64,           // How much RAM for model
    pub human_activity: HumanActivity,
    pub auto_adjust: bool,         // Automatically reduce when human is busy
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HumanActivity {
    /// Not at the computer
    Away,
    /// At computer, idle
    Idle,
    /// Typing / browsing (light use)
    LightUse,
    /// Heavy work (compiling, rendering, gaming)
    HeavyUse,
    /// Actively talking to Aura
    Conversing,
}

/// The model router's decision
#[derive(Debug, Clone)]
pub struct RoutingDecision {
    pub model: String,
    pub source: ModelSource,
    pub reason: String,
    pub estimated_latency_ms: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelSource {
    LocalModel,
    CloudApi,
    PatternMatch,
    MeshNode,  // Another computer in the Aura Mesh
}

impl AuraEngine {
    /// Create a new engine (starts in PatternMatchOnly mode)
    pub fn new() -> Self {
        AuraEngine {
            state: EngineState::PatternMatchOnly,
            local_models: Vec::new(),
            cloud_providers: Vec::new(),
            hardware_profile: HardwareProfile::detect(),
            resource_budget: ResourceBudget {
                max_cpu_percent: 40,
                max_ram_mb: 0,
                human_activity: HumanActivity::Idle,
                auto_adjust: true,
            },
        }
    }

    /// Route a query to the best available model
    pub fn route(&self, query: &str, capability_needed: ModelCapability) -> RoutingDecision {
        // Priority: Local model → Cloud API → Pattern matching
        
        // 1. Check local models
        for model in &self.local_models {
            if model.loaded && model.capabilities.contains(&capability_needed) {
                return RoutingDecision {
                    model: model.name.clone(),
                    source: ModelSource::LocalModel,
                    reason: String::from("Local model available"),
                    estimated_latency_ms: (1000.0 / model.tokens_per_second) as u32 * 50,
                };
            }
        }

        // 2. Check cloud providers
        for provider in &self.cloud_providers {
            if provider.enabled && !provider.models.is_empty() {
                return RoutingDecision {
                    model: provider.models[0].clone(),
                    source: ModelSource::CloudApi,
                    reason: String::from("Using cloud API"),
                    estimated_latency_ms: 500,
                };
            }
        }

        // 3. Fall back to pattern matching
        RoutingDecision {
            model: String::from("pattern-match"),
            source: ModelSource::PatternMatch,
            reason: String::from("No AI models available — using keyword matching"),
            estimated_latency_ms: 1,
        }
    }
}

impl HardwareProfile {
    /// Detect hardware capabilities
    pub fn detect() -> Self {
        // For now, use basic detection
        // TODO: Full CPUID enumeration, PCI scan for GPU, memory mapping
        let ram_mb = 512; // Will be filled from boot_info
        
        let tier = match ram_mb {
            0..=3999 => ModelTier::Tiny,
            4000..=7999 => ModelTier::Small,
            8000..=15999 => ModelTier::Medium,
            16000..=31999 => ModelTier::Large,
            _ => ModelTier::VeryLarge,
        };

        HardwareProfile {
            cpu_cores: 1, // TODO: detect from ACPI
            cpu_freq_mhz: 0,
            ram_total_mb: ram_mb,
            ram_available_mb: ram_mb / 2,
            gpu_detected: false,
            gpu_vram_mb: 0,
            gpu_compute_capability: None,
            disk_speed_mbps: 0,
            recommended_tier: tier,
        }
    }
}

/// System-wide engine instance
static ENGINE: spin::Lazy<spin::Mutex<AuraEngine>> = spin::Lazy::new(|| {
    spin::Mutex::new(AuraEngine::new())
});

/// Get the Aura Engine status for display
pub fn status_string() -> String {
    let engine = ENGINE.lock();
    alloc::format!(
        "Aura Engine: {:?} | Tier: {:?} | Models: {} local, {} cloud",
        engine.state,
        engine.hardware_profile.recommended_tier,
        engine.local_models.len(),
        engine.cloud_providers.len()
    )
}
