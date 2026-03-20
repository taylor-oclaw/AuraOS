extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AiRuntimeAbstract {
    name: String,
    version: u32,
    features: Vec<String>,
    memory_usage: usize,
    processing_power: f32,
}

impl AiRuntimeAbstract {
    pub fn new(name: &str, version: u32) -> Self {
        AiRuntimeAbstract {
            name: String::from(name),
            version,
            features: Vec::new(),
            memory_usage: 0,
            processing_power: 0.0,
        }
    }

    pub fn add_feature(&mut self, feature: &str) {
        self.features.push(String::from(feature));
    }

    pub fn get_features(&self) -> &[String] {
        &self.features
    }

    pub fn set_memory_usage(&mut self, usage: usize) {
        self.memory_usage = usage;
    }

    pub fn get_memory_usage(&self) -> usize {
        self.memory_usage
    }

    pub fn set_processing_power(&mut self, power: f32) {
        self.processing_power = power;
    }

    pub fn get_processing_power(&self) -> f32 {
        self.processing_power
    }
}
