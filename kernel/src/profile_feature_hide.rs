extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct ProfileFeatureHide {
    hidden_features: Vec<String>,
}

impl ProfileFeatureHide {
    pub fn new() -> Self {
        ProfileFeatureHide {
            hidden_features: Vec::new(),
        }
    }

    pub fn add_feature(&mut self, feature: &str) {
        self.hidden_features.push(feature.to_string());
    }

    pub fn remove_feature(&mut self, feature: &str) {
        if let Some(index) = self.hidden_features.iter().position(|f| f == feature) {
            self.hidden_features.remove(index);
        }
    }

    pub fn is_feature_hidden(&self, feature: &str) -> bool {
        self.hidden_features.contains(&feature.to_string())
    }

    pub fn list_hidden_features(&self) -> Vec<String> {
        self.hidden_features.clone()
    }

    pub fn clear_hidden_features(&mut self) {
        self.hidden_features.clear();
    }
}
