extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ProfileModeWifiTrigger {
    profiles: Vec<String>,
    current_profile: usize,
}

impl ProfileModeWifiTrigger {
    pub fn new() -> Self {
        ProfileModeWifiTrigger {
            profiles: Vec::new(),
            current_profile: 0,
        }
    }

    pub fn add_profile(&mut self, profile_name: &str) {
        self.profiles.push(String::from(profile_name));
    }

    pub fn remove_profile(&mut self, profile_name: &str) -> bool {
        if let Some(index) = self.profiles.iter().position(|p| p == profile_name) {
            self.profiles.remove(index);
            true
        } else {
            false
        }
    }

    pub fn switch_to_next_profile(&mut self) {
        if !self.profiles.is_empty() {
            self.current_profile = (self.current_profile + 1) % self.profiles.len();
        }
    }

    pub fn get_current_profile(&self) -> Option<&str> {
        self.profiles.get(self.current_profile).map(|s| s.as_str())
    }

    pub fn list_profiles(&self) -> Vec<&str> {
        self.profiles.iter().map(|p| p.as_str()).collect()
    }
}
