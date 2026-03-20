extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentAPIGenerator {
    // Example fields
    name: String,
    version: u32,
    features: Vec<String>,
}

impl AgentAPIGenerator {
    pub fn new(name: &str, version: u32) -> Self {
        AgentAPIGenerator {
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

    pub fn list_features(&self) -> &[String] {
        &self.features
    }

    pub fn has_feature(&self, feature: &str) -> bool {
        self.features.contains(&String::from(feature))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_api_generator() {
        let mut generator = AgentAPIGenerator::new("TestAgent", 1);
        assert_eq!(generator.get_name(), "TestAgent");
        assert_eq!(generator.get_version(), 1);
        assert!(!generator.has_feature("FeatureA"));

        generator.add_feature("FeatureA");
        assert!(generator.has_feature("FeatureA"));
        assert_eq!(generator.list_features().len(), 1);

        generator.add_feature("FeatureB");
        assert_eq!(generator.list_features().len(), 2);
    }
}
