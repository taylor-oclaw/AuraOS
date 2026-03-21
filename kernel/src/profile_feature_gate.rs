extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ProfileFeatureGate {
    features: Vec<String>,
}

impl ProfileFeatureGate {
    pub fn new() -> Self {
        ProfileFeatureGate {
            features: Vec::new(),
        }
    }

    pub fn add_feature(&mut self, feature: String) {
        if !self.features.contains(&feature) {
            self.features.push(feature);
        }
    }

    pub fn remove_feature(&mut self, feature: &str) {
        self.features.retain(|f| f != feature);
    }

    pub fn has_feature(&self, feature: &str) -> bool {
        self.features.iter().any(|f| f == feature)
    }

    pub fn list_features(&self) -> Vec<String> {
        self.features.clone()
    }

    pub fn clear_features(&mut self) {
        self.features.clear();
    }
}
