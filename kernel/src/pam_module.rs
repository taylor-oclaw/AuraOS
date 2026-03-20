extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PamModule {
    name: String,
    version: u32,
    features: Vec<String>,
}

impl PamModule {
    pub fn new(name: &str, version: u32) -> Self {
        PamModule {
            name: String::from(name),
            version,
            features: Vec::new(),
        }
    }

    pub fn add_feature(&mut self, feature: &str) {
        self.features.push(String::from(feature));
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_version(&self) -> u32 {
        self.version
    }

    pub fn has_feature(&self, feature: &str) -> bool {
        self.features.contains(&String::from(feature))
    }

    pub fn list_features(&self) -> Vec<&str> {
        self.features.iter().map(|f| f.as_str()).collect()
    }
}
