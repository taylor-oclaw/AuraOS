extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct RelationshipPreferenceLearn {
    preferences: Vec<String>,
}

impl RelationshipPreferenceLearn {
    pub fn new() -> Self {
        RelationshipPreferenceLearn {
            preferences: Vec::new(),
        }
    }

    pub fn add_preference(&mut self, preference: String) {
        if !self.preferences.contains(&preference) {
            self.preferences.push(preference);
        }
    }

    pub fn remove_preference(&mut self, preference: &str) -> bool {
        let index = self.preferences.iter().position(|p| p == preference);
        if let Some(idx) = index {
            self.preferences.remove(idx);
            true
        } else {
            false
        }
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
