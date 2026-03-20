extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AIKernelModule {
    name: String,
    version: u32,
    features: Vec<String>,
}

impl AIKernelModule {
    pub fn new(name: &str, version: u32) -> Self {
        AIKernelModule {
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
    fn test_ai_kernel_module() {
        let mut module = AIKernelModule::new("AI-Tool", 1);
        assert_eq!(module.get_name(), "AI-Tool");
        assert_eq!(module.get_version(), 1);
        assert!(!module.has_feature("Machine Learning"));

        module.add_feature("Machine Learning");
        assert!(module.has_feature("Machine Learning"));
        assert_eq!(module.list_features().len(), 1);

        module.add_feature("Natural Language Processing");
        assert_eq!(module.list_features().len(), 2);
    }
}
