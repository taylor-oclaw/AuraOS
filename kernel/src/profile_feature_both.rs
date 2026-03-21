extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut profile = ProfileFeatureBoth::new(String::from("AI-Native OS"));
    profile.add_feature("Machine Learning");
    profile.add_feature("Natural Language Processing");
    profile.add_feature("Computer Vision");
    profile.add_feature("Robotics");
    profile.add_feature("Blockchain");

    // Example usage of the methods

    if let Some(feature) = profile.get_feature(2) {
    }

    profile.remove_feature("Natural Language Processing");

    loop {}
}

pub struct ProfileFeatureBoth {
    name: String,
    features: Vec<String>,
}

impl ProfileFeatureBoth {
    pub fn new(name: String) -> Self {
        ProfileFeatureBoth {
            name,
            features: Vec::new(),
        }
    }

    pub fn add_feature(&mut self, feature: &str) {
        self.features.push(feature.to_string());
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn feature_count(&self) -> usize {
        self.features.len()
    }

    pub fn get_feature(&self, index: usize) -> Option<&str> {
        self.features.get(index).map(|s| s.as_str())
    }

    pub fn remove_feature(&mut self, feature: &str) {
        if let Some(pos) = self.features.iter().position(|f| f == feature) {
            self.features.remove(pos);
        }
    }
}
