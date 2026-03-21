extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialization code for the module
}

pub extern "C" fn rust_ffi_exit() {
    // Cleanup code for the module
}

pub struct AuraAssistantBrief {
    name: String,
    version: String,
    features: Vec<String>,
    active: bool,
}

impl AuraAssistantBrief {
    pub fn new(name: &str, version: &str) -> Self {
        AuraAssistantBrief {
            name: String::from(name),
            version: String::from(version),
            features: Vec::new(),
            active: false,
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
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

    pub fn list_features(&self) -> Vec<String> {
        self.features.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aura_assistant_brief() {
        let mut assistant = AuraAssistantBrief::new("Aura Assistant", "1.0");
        assert_eq!(assistant.name, "Aura Assistant");
        assert_eq!(assistant.version, "1.0");
        assert!(!assistant.active);

        assistant.activate();
        assert!(assistant.active);

        assistant.deactivate();
        assert!(!assistant.active);

        assistant.add_feature("Natural Language Processing");
        assistant.add_feature("Machine Learning");
        let features = assistant.list_features();
        assert_eq!(features.len(), 2);
        assert!(features.contains(&String::from("Natural Language Processing")));
        assert!(features.contains(&String::from("Machine Learning")));

        assert!(assistant.remove_feature("Natural Language Processing"));
        let features = assistant.list_features();
        assert_eq!(features.len(), 1);
        assert!(!features.contains(&String::from("Natural Language Processing")));
    }
}
