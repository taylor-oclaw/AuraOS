extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct PeoplePreferenceEvolve {
    preferences: Vec<String>,
}

impl PeoplePreferenceEvolve {
    pub fn new() -> Self {
        PeoplePreferenceEvolve {
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
        self.preferences.contains(&preference.to_string())
    }

    pub fn list_preferences(&self) -> Vec<String> {
        self.preferences.clone()
    }

    pub fn update_preference(&mut self, old_preference: &str, new_preference: String) {
        if let Some(index) = self.preferences.iter().position(|p| p == old_preference) {
            self.preferences[index] = new_preference;
        }
    }
}
