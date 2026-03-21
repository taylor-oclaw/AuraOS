extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct ProfileFeatureUnlock {
    features: Vec<String>,
    enabled_features: Vec<String>,
}

impl ProfileFeatureUnlock {
    pub fn new() -> Self {
        ProfileFeatureUnlock {
            features: Vec::new(),
            enabled_features: Vec::new(),
        }
    }

    pub fn add_feature(&mut self, feature_name: &str) {
        if !self.features.contains(&String::from(feature_name)) {
            self.features.push(String::from(feature_name));
        }
    }

    pub fn enable_feature(&mut self, feature_name: &str) -> bool {
        if self.features.contains(&String::from(feature_name))
            && !self.enabled_features.contains(&String::from(feature_name))
        {
            self.enabled_features.push(String::from(feature_name));
            true
        } else {
            false
        }
    }

    pub fn disable_feature(&mut self, feature_name: &str) -> bool {
        if let Some(index) = self.enabled_features.iter().position(|f| f == feature_name) {
            self.enabled_features.remove(index);
            true
        } else {
            false
        }
    }

    pub fn is_feature_enabled(&self, feature_name: &str) -> bool {
        self.enabled_features.contains(&String::from(feature_name))
    }

    pub fn list_all_features(&self) -> Vec<String> {
        self.features.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_feature_unlock() {
        let mut pfu = ProfileFeatureUnlock::new();
        assert_eq!(pfu.list_all_features().len(), 0);

        pfu.add_feature("AI_Enhancement");
        pfu.add_feature("SpeedBoost");
        assert_eq!(pfu.list_all_features().len(), 2);
        assert_eq!(pfu.is_feature_enabled("AI_Enhancement"), false);

        assert!(pfu.enable_feature("AI_Enhancement"));
        assert!(!pfu.enable_feature("AI_Enhancement")); // Already enabled
        assert_eq!(pfu.is_feature_enabled("AI_Enhancement"), true);

        assert!(pfu.disable_feature("AI_Enhancement"));
        assert!(!pfu.disable_feature("AI_Enhancement")); // Already disabled
        assert_eq!(pfu.is_feature_enabled("AI_Enhancement"), false);
    }
}
