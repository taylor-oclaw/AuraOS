extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod ai_extension_config {
    use super::*;

    pub struct AIExtensionConfig {
        config_name: String,
        settings: Vec<(String, String)>,
    }

    impl AIExtensionConfig {
        pub fn new(config_name: &str) -> Self {
            AIExtensionConfig {
                config_name: String::from(config_name),
                settings: Vec::new(),
            }
        }

        pub fn add_setting(&mut self, key: &str, value: &str) {
            self.settings.push((String::from(key), String::from(value)));
        }

        pub fn get_setting(&self, key: &str) -> Option<&String> {
            self.settings.iter().find_map(|(k, v)| if k == key { Some(v) } else { None })
        }

        pub fn remove_setting(&mut self, key: &str) {
            self.settings.retain(|(k, _)| k != key);
        }

        pub fn list_settings(&self) -> Vec<&String> {
            self.settings.iter().map(|(_, v)| v).collect()
        }

        pub fn config_name(&self) -> &String {
            &self.config_name
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ai_extension_config::*;

    #[test]
    fn test_ai_extension_config() {
        let mut config = AIExtensionConfig::new("TestConfig");
        assert_eq!(config.config_name(), "TestConfig");

        config.add_setting("setting1", "value1");
        config.add_setting("setting2", "value2");

        assert_eq!(config.get_setting("setting1"), Some(&String::from("value1")));
        assert_eq!(config.list_settings(), vec![&String::from("value1"), &String::from("value2")]);

        config.remove_setting("setting1");
        assert_eq!(config.get_setting("setting1"), None);
        assert_eq!(config.list_settings(), vec![&String::from("value2")]);
    }
}
