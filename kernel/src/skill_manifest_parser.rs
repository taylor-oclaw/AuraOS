extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SkillManifest {
    name: String,
    version: u32,
    description: String,
    dependencies: Vec<String>,
    features: Vec<String>,
}

impl SkillManifest {
    pub fn new(name: &str, version: u32, description: &str) -> Self {
        SkillManifest {
            name: String::from(name),
            version,
            description: String::from(description),
            dependencies: Vec::new(),
            features: Vec::new(),
        }
    }

    pub fn add_dependency(&mut self, dependency: &str) {
        self.dependencies.push(String::from(dependency));
    }

    pub fn remove_dependency(&mut self, dependency: &str) {
        self.dependencies.retain(|d| d != dependency);
    }

    pub fn has_feature(&self, feature: &str) -> bool {
        self.features.contains(&String::from(feature))
    }

    pub fn add_feature(&mut self, feature: &str) {
        if !self.has_feature(feature) {
            self.features.push(String::from(feature));
        }
    }

    pub fn remove_feature(&mut self, feature: &str) {
        self.features.retain(|f| f != feature);
    }

    pub fn get_dependencies(&self) -> Vec<String> {
        self.dependencies.clone()
    }

    pub fn get_features(&self) -> Vec<String> {
        self.features.clone()
    }
}
