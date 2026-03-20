extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AIKernelModule {
    name: String,
    version: String,
    features: Vec<String>,
    initialized: bool,
}

impl AIKernelModule {
    pub fn new(name: &str, version: &str) -> Self {
        AIKernelModule {
            name: String::from(name),
            version: String::from(version),
            features: Vec::new(),
            initialized: false,
        }
    }

    pub fn initialize(&mut self) {
        if !self.initialized {
            // Simulate initialization logic
            self.initialized = true;
        } else {
        }
    }

    pub fn add_feature(&mut self, feature: &str) {
        self.features.push(String::from(feature));
    }

    pub fn remove_feature(&mut self, feature: &str) -> bool {
        if let Some(index) = self.features.iter().position(|f| f == feature) {
            self.features.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_features(&self) -> Vec<String> {
        self.features.clone()
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}
