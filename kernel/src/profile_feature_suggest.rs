extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ProfileFeatureSuggest {
    user_profile: String,
    features: Vec<String>,
}

impl ProfileFeatureSuggest {
    pub fn new(user_profile: &str) -> Self {
        ProfileFeatureSuggest {
            user_profile: String::from(user_profile),
            features: Vec::new(),
        }
    }

    pub fn add_feature(&mut self, feature: &str) {
        if !self.features.contains(&String::from(feature)) {
            self.features.push(String::from(feature));
        }
    }

    pub fn remove_feature(&mut self, feature: &str) {
        if let Some(index) = self.features.iter().position(|f| f == feature) {
            self.features.remove(index);
        }
    }

    pub fn get_features(&self) -> Vec<String> {
        self.features.clone()
    }

    pub fn suggest_feature(&self) -> Option<&String> {
        // Simple suggestion logic: return the first feature in the list
        self.features.first()
    }

    pub fn update_profile(&mut self, new_profile: &str) {
        self.user_profile = String::from(new_profile);
    }
}
