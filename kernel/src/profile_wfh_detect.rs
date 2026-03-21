extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ProfileWFHDetect {
    profiles: Vec<String>,
}

impl ProfileWFHDetect {
    pub fn new() -> Self {
        ProfileWFHDetect {
            profiles: Vec::new(),
        }
    }

    pub fn add_profile(&mut self, profile_name: &str) {
        self.profiles.push(String::from(profile_name));
    }

    pub fn remove_profile(&mut self, profile_name: &str) {
        if let Some(index) = self.profiles.iter().position(|p| p == profile_name) {
            self.profiles.remove(index);
        }
    }

    pub fn list_profiles(&self) -> Vec<String> {
        self.profiles.clone()
    }

    pub fn is_profile_active(&self, profile_name: &str) -> bool {
        self.profiles.contains(&String::from(profile_name))
    }

    pub fn clear_profiles(&mut self) {
        self.profiles.clear();
    }
}
