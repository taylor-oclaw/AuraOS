//! AuraOS AI Companion — LLM Runtime
//! 
//! Runs a local language model for:
//! - Natural language understanding
//! - Intent classification
//! - Context-aware suggestions
//! - Conversational interaction
//! - Tool use (controlling the OS)
//! 
//! Uses candle (Rust-native ML) or llama.cpp bindings.
//! Models run 100% locally — no cloud, no data leaving the device.

/// Supported model formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelFormat {
    GGUF,       // llama.cpp format (quantized, efficient)
    SafeTensors, // HuggingFace format
    ONNX,       // Cross-platform inference
}

/// Model configuration
#[derive(Debug, Clone)]
pub struct ModelConfig {
    pub path: String,
    pub format: ModelFormat,
    pub context_length: usize,
    pub n_gpu_layers: i32,      // -1 = all on GPU
    pub threads: usize,
    pub temperature: f32,
    pub top_p: f32,
}

/// A message in the conversation
#[derive(Debug, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

/// Tools the companion can use to control the OS
#[derive(Debug, Clone)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub parameters: Vec<ToolParam>,
}

#[derive(Debug, Clone)]
pub struct ToolParam {
    pub name: String,
    pub param_type: String,
    pub description: String,
    pub required: bool,
}

/// Built-in OS tools for the companion
pub fn system_tools() -> Vec<Tool> {
    vec![
        Tool {
            name: "open_surface".into(),
            description: "Open a surface (application) by name or intent".into(),
            parameters: vec![
                ToolParam { name: "query".into(), param_type: "string".into(), description: "What to open".into(), required: true },
            ],
        },
        Tool {
            name: "find_file".into(),
            description: "Search for files by name or content".into(),
            parameters: vec![
                ToolParam { name: "query".into(), param_type: "string".into(), description: "Search query".into(), required: true },
            ],
        },
        Tool {
            name: "system_setting".into(),
            description: "Change a system setting".into(),
            parameters: vec![
                ToolParam { name: "setting".into(), param_type: "string".into(), description: "Setting name".into(), required: true },
                ToolParam { name: "value".into(), param_type: "string".into(), description: "New value".into(), required: true },
            ],
        },
        Tool {
            name: "send_message".into(),
            description: "Send a message to a contact".into(),
            parameters: vec![
                ToolParam { name: "to".into(), param_type: "string".into(), description: "Recipient".into(), required: true },
                ToolParam { name: "message".into(), param_type: "string".into(), description: "Message text".into(), required: true },
            ],
        },
        Tool {
            name: "set_reminder".into(),
            description: "Set a timed reminder".into(),
            parameters: vec![
                ToolParam { name: "text".into(), param_type: "string".into(), description: "Reminder text".into(), required: true },
                ToolParam { name: "when".into(), param_type: "string".into(), description: "When to remind".into(), required: true },
            ],
        },
    ]
}
