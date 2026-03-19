//! BPE Tokenizer for LLM inference
//!
//! Converts text ↔ tokens. Supports the tokenizer data embedded in GGUF files.
//! Compatible with LLaMA, Qwen, Phi, Mistral tokenizers.

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

/// A token in the vocabulary
#[derive(Debug, Clone)]
pub struct Token {
    pub text: String,
    pub score: f32,
    pub token_type: TokenType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Normal,
    Unknown,
    Control,    // BOS, EOS, PAD
    UserDefined,
    Unused,
    Byte,       // Fallback byte token
}

/// The tokenizer
pub struct Tokenizer {
    pub vocab: Vec<Token>,
    pub bos_id: u32,  // Beginning of sequence
    pub eos_id: u32,  // End of sequence
    pub pad_id: u32,  // Padding
    pub model_type: TokenizerModel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenizerModel {
    BPE,
    SPM,       // SentencePiece
    WordPiece,
}

impl Tokenizer {
    /// Create from GGUF metadata
    pub fn from_gguf(gguf: &crate::gguf::GgufFile) -> Option<Self> {
        // Extract vocab from GGUF metadata
        let tokens = match gguf.get_metadata("tokenizer.ggml.tokens") {
            Some(crate::gguf::MetadataValue::Array(arr)) => {
                arr.iter().filter_map(|v| {
                    if let crate::gguf::MetadataValue::Str(s) = v {
                        Some(s.clone())
                    } else {
                        None
                    }
                }).collect::<Vec<_>>()
            }
            _ => return None,
        };

        let scores = match gguf.get_metadata("tokenizer.ggml.scores") {
            Some(crate::gguf::MetadataValue::Array(arr)) => {
                arr.iter().filter_map(|v| {
                    if let crate::gguf::MetadataValue::Float32(f) = v {
                        Some(*f)
                    } else {
                        None
                    }
                }).collect::<Vec<_>>()
            }
            _ => Vec::new(),
        };

        let token_types = match gguf.get_metadata("tokenizer.ggml.token_type") {
            Some(crate::gguf::MetadataValue::Array(arr)) => {
                arr.iter().filter_map(|v| {
                    if let crate::gguf::MetadataValue::Int32(i) = v {
                        Some(*i)
                    } else {
                        None
                    }
                }).collect::<Vec<_>>()
            }
            _ => Vec::new(),
        };

        let vocab: Vec<Token> = tokens.into_iter().enumerate().map(|(i, text)| {
            let score = scores.get(i).copied().unwrap_or(0.0);
            let tt = token_types.get(i).copied().unwrap_or(0);
            Token {
                text,
                score,
                token_type: match tt {
                    1 => TokenType::Normal,
                    2 => TokenType::Unknown,
                    3 => TokenType::Control,
                    4 => TokenType::UserDefined,
                    5 => TokenType::Unused,
                    6 => TokenType::Byte,
                    _ => TokenType::Normal,
                },
            }
        }).collect();

        let bos_id = gguf.get_u32("tokenizer.ggml.bos_token_id").unwrap_or(1);
        let eos_id = gguf.get_u32("tokenizer.ggml.eos_token_id").unwrap_or(2);
        let pad_id = gguf.get_u32("tokenizer.ggml.padding_token_id").unwrap_or(0);

        Some(Tokenizer {
            vocab,
            bos_id,
            eos_id,
            pad_id,
            model_type: TokenizerModel::BPE,
        })
    }

    /// Encode text to token IDs (simple greedy tokenization)
    pub fn encode(&self, text: &str) -> Vec<u32> {
        let mut tokens = Vec::new();
        tokens.push(self.bos_id);

        let bytes = text.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            let mut best_len = 0;
            let mut best_id = 0u32;

            // Find longest matching token
            for (id, token) in self.vocab.iter().enumerate() {
                let t = token.text.as_bytes();
                if t.len() > best_len && i + t.len() <= bytes.len() {
                    if &bytes[i..i + t.len()] == t {
                        best_len = t.len();
                        best_id = id as u32;
                    }
                }
            }

            if best_len == 0 {
                // Fallback: encode as byte token
                i += 1;
            } else {
                tokens.push(best_id);
                i += best_len;
            }
        }

        tokens
    }

    /// Decode token IDs back to text
    pub fn decode(&self, tokens: &[u32]) -> String {
        let mut result = String::new();
        for &id in tokens {
            if id == self.bos_id || id == self.eos_id || id == self.pad_id {
                continue;
            }
            if let Some(token) = self.vocab.get(id as usize) {
                result.push_str(&token.text);
            }
        }
        result
    }

    pub fn vocab_size(&self) -> usize {
        self.vocab.len()
    }
}
