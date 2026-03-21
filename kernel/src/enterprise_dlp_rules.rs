extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

mod enterprise_dlp_rules {
    use super::*;

    pub struct DLPConfig {
        rules: Vec<String>,
        enabled: bool,
    }

    impl DLPConfig {
        pub fn new() -> Self {
            DLPConfig {
                rules: Vec::new(),
                enabled: false,
            }
        }

        pub fn enable(&mut self) {
            self.enabled = true;
        }

        pub fn disable(&mut self) {
            self.enabled = false;
        }

        pub fn is_enabled(&self) -> bool {
            self.enabled
        }

        pub fn add_rule(&mut self, rule: String) {
            if !self.rules.contains(&rule) {
                self.rules.push(rule);
            }
        }

        pub fn remove_rule(&mut self, rule: &str) {
            self.rules.retain(|r| r != rule);
        }

        pub fn list_rules(&self) -> Vec<String> {
            self.rules.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::enterprise_dlp_rules::*;

    #[test]
    fn test_dlp_config() {
        let mut config = DLPConfig::new();
        assert!(!config.is_enabled());

        config.enable();
        assert!(config.is_enabled());

        config.disable();
        assert!(!config.is_enabled());

        config.add_rule(String::from("rule1"));
        config.add_rule(String::from("rule2"));
        assert_eq!(config.list_rules(), vec![String::from("rule1"), String::from("rule2")]);

        config.remove_rule("rule1");
        assert_eq!(config.list_rules(), vec![String::from("rule2")]);
    }
}
