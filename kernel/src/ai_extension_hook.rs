extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct AIExtensionHook {
    name: String,
    version: u32,
    features: Vec<String>,
    enabled: bool,
}

impl AIExtensionHook {
    pub fn new(name: &str, version: u32) -> Self {
        AIExtensionHook {
            name: String::from(name),
            version,
            features: Vec::new(),
            enabled: false,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
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
    fn test_ai_extension_hook() {
        let mut hook = AIExtensionHook::new("TestHook", 1);
        assert_eq!(hook.name, "TestHook");
        assert_eq!(hook.version, 1);
        assert!(!hook.enabled);

        hook.enable();
        assert!(hook.enabled);

        hook.disable();
        assert!(!hook.enabled);

        hook.add_feature("Feature1");
        hook.add_feature("Feature2");
        let features = hook.list_features();
        assert_eq!(features.len(), 2);
        assert!(features.contains(&String::from("Feature1")));
        assert!(features.contains(&String::from("Feature2")));

        assert!(hook.remove_feature("Feature1"));
        let features = hook.list_features();
        assert_eq!(features.len(), 1);
        assert!(!features.contains(&String::from("Feature1")));
        assert!(features.contains(&String::from("Feature2")));

        assert!(!hook.remove_feature("Feature3"));
    }
}
