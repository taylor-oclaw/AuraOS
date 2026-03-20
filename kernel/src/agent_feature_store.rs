extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentFeatureStore {
    features: Vec<String>,
}

impl AgentFeatureStore {
    pub fn new() -> Self {
        AgentFeatureStore {
            features: Vec::new(),
        }
    }

    pub fn add_feature(&mut self, feature: String) {
        if !self.features.contains(&feature) {
            self.features.push(feature);
        }
    }

    pub fn remove_feature(&mut self, feature: &str) -> bool {
        let index = self.features.iter().position(|f| f == feature);
        match index {
            Some(i) => {
                self.features.remove(i);
                true
            }
            None => false,
        }
    }

    pub fn has_feature(&self, feature: &str) -> bool {
        self.features.contains(&String::from(feature))
    }

    pub fn list_features(&self) -> Vec<String> {
        self.features.clone()
    }

    pub fn clear_features(&mut self) {
        self.features.clear();
    }
}
