extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct ModelInfo {
    pub name: String,
    pub version: String,
    pub size_mb: u64,
    pub quantization: String,
    pub capabilities: Vec<String>,
    pub loaded: bool,
    pub downloads: u64,
    pub license: String,
    pub cost_tier: u8
}

pub struct ModelRegistry {
    pub models: Vec<ModelInfo>,
    pub download_queue: Vec<String>
}

impl ModelRegistry {
    pub fn new() -> Self {
        let models = vec![
            ModelInfo {
                name: String::from("aura-nano"),
                version: String::from("1.0"),
                size_mb: 200,
                quantization: String::from("Q4_K_M"),
                capabilities: vec![String::from("classify")],
                loaded: false,
                downloads: 0,
                license: String::from("MIT"),
                cost_tier: 1
            },
            ModelInfo {
                name: String::from("aura-base"),
                version: String::from("1.0"),
                size_mb: 2000,
                quantization: String::from("Q5_K_M"),
                capabilities: vec![String::from("chat"), String::from("summarize")],
                loaded: false,
                downloads: 0,
                license: String::from("MIT"),
                cost_tier: 3
            },
            ModelInfo {
                name: String::from("aura-pro"),
                version: String::from("1.0"),
                size_mb: 8000,
                quantization: String::from("Q6_K"),
                capabilities: vec![String::from("code"), String::from("reason"), String::from("vision")],
                loaded: false,
                downloads: 0,
                license: String::from("Commercial"),
                cost_tier: 5
            }
        ];
        Self { models, download_queue: Vec::new() }
    }

    pub fn find_by_capability(&self, cap: &str) -> Vec<&ModelInfo> {
        self.models.iter().filter(|m| m.capabilities.iter().any(|c| c == cap)).collect()
    }

    pub fn cheapest_for(&self, cap: &str) -> Option<&ModelInfo> {
        self.find_by_capability(cap).into_iter().min_by_key(|m| m.cost_tier)
    }

    pub fn load(&mut self, name: &str) -> bool {
        if let Some(m) = self.models.iter_mut().find(|m| m.name == name) {
            m.loaded = true;
            true
        } else {
            false
        }
    }

    pub fn loaded_models(&self) -> Vec<&ModelInfo> {
        self.models.iter().filter(|m| m.loaded).collect()
    }

    pub fn total_loaded_mb(&self) -> u64 {
        self.models.iter().filter(|m| m.loaded).map(|m| m.size_mb).sum()
    }
}
