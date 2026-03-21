extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_std]
mod rel_email_pref_detect {
    use core::str::FromStr;

    pub struct EmailPreferenceDetector {
        preferences: Vec<String>,
    }

    impl EmailPreferenceDetector {
        pub fn new() -> Self {
            EmailPreferenceDetector {
                preferences: Vec::new(),
            }
        }

        pub fn add_preference(&mut self, preference: &str) {
            if !self.preferences.contains(&preference.to_string()) {
                self.preferences.push(preference.to_string());
            }
        }

        pub fn remove_preference(&mut self, preference: &str) {
            self.preferences.retain(|p| p != preference);
        }

        pub fn has_preference(&self, preference: &str) -> bool {
            self.preferences.contains(&preference.to_string())
        }

        pub fn list_preferences(&self) -> Vec<String> {
            self.preferences.clone()
        }

        pub fn clear_preferences(&mut self) {
            self.preferences.clear();
        }
    }
}
