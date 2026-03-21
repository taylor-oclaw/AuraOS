extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct SocialMediaPreferences {
    preferences: Vec<String>,
}

impl SocialMediaPreferences {
    pub fn new() -> Self {
        SocialMediaPreferences {
            preferences: Vec::new(),
        }
    }

    pub fn add_preference(&mut self, preference: String) {
        if !self.preferences.contains(&preference) {
            self.preferences.push(preference);
        }
    }

    pub fn remove_preference(&mut self, preference: &str) {
        self.preferences.retain(|p| p != preference);
    }

    pub fn has_preference(&self, preference: &str) -> bool {
        self.preferences.contains(&String::from(preference))
    }

    pub fn list_preferences(&self) -> Vec<String> {
        self.preferences.clone()
    }

    pub fn clear_preferences(&mut self) {
        self.preferences.clear();
    }
}
