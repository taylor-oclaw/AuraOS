extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ProfileCoffeeShopScreenGuard {
    user_profiles: Vec<String>,
    active_profile: Option<usize>,
}

impl ProfileCoffeeShopScreenGuard {
    pub fn new() -> Self {
        ProfileCoffeeShopScreenGuard {
            user_profiles: Vec::new(),
            active_profile: None,
        }
    }

    pub fn add_profile(&mut self, profile_name: &str) {
        let profile = String::from(profile_name);
        self.user_profiles.push(profile);
    }

    pub fn remove_profile(&mut self, profile_index: usize) -> Option<String> {
        if profile_index < self.user_profiles.len() {
            Some(self.user_profiles.remove(profile_index))
        } else {
            None
        }
    }

    pub fn activate_profile(&mut self, profile_index: usize) -> bool {
        if profile_index < self.user_profiles.len() {
            self.active_profile = Some(profile_index);
            true
        } else {
            false
        }
    }

    pub fn deactivate_profile(&mut self) {
        self.active_profile = None;
    }

    pub fn get_active_profile_name(&self) -> Option<&str> {
        if let Some(index) = self.active_profile {
            Some(&self.user_profiles[index])
        } else {
            None
        }
    }
}
