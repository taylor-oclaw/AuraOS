extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct AccessAutismSensory {
    name: String,
    features: Vec<String>,
}

impl AccessAutismSensory {
    pub fn new(name: &str) -> Self {
        AccessAutismSensory {
            name: String::from(name),
            features: Vec::new(),
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

    pub fn has_feature(&self, feature: &str) -> bool {
        self.features.contains(&String::from(feature))
    }

    pub fn list_features(&self) -> Vec<String> {
        self.features.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
