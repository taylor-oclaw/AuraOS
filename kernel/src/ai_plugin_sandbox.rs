extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct AIPluginSandbox {
    name: String,
    version: u32,
    features: Vec<String>,
    active: bool,
}

impl AIPluginSandbox {
    pub fn new(name: &str, version: u32) -> Self {
        AIPluginSandbox {
            name: String::from(name),
            version,
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

    pub fn remove_feature(&mut self, feature: &str) {
        self.features.retain(|f| f != feature);
    }

    pub fn list_features(&self) -> Vec<String> {
        self.features.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_plugin_sandbox() {
        let mut sandbox = AIPluginSandbox::new("TestPlugin", 1);
        assert_eq!(sandbox.name, "TestPlugin");
        assert_eq!(sandbox.version, 1);
        assert!(!sandbox.active);

        sandbox.activate();
        assert!(sandbox.active);

        sandbox.deactivate();
        assert!(!sandbox.active);

        sandbox.add_feature("Feature1");
        sandbox.add_feature("Feature2");
        let features = sandbox.list_features();
        assert_eq!(features.len(), 2);
        assert!(features.contains(&String::from("Feature1")));
        assert!(features.contains(&String::from("Feature2")));

        sandbox.remove_feature("Feature1");
        let features = sandbox.list_features();
        assert_eq!(features.len(), 1);
        assert!(!features.contains(&String::from("Feature1")));
        assert!(features.contains(&String::from("Feature2")));
    }
}
