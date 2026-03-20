extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct MetalCompat {
    name: String,
    version: u32,
    features: Vec<String>,
    enabled: bool,
}

impl MetalCompat {
    pub fn new(name: &str, version: u32) -> Self {
        MetalCompat {
            name: String::from(name),
            version,
            features: Vec::new(),
            enabled: false,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
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
}
