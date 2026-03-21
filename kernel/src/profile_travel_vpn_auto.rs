extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct ProfileTravelVPNAuto {
    profiles: Vec<String>,
    current_profile: Option<usize>,
}

impl ProfileTravelVPNAuto {
    pub fn new() -> Self {
        ProfileTravelVPNAuto {
            profiles: Vec::new(),
            current_profile: None,
        }
    }

    pub fn add_profile(&mut self, profile_name: &str) {
        let profile = String::from(profile_name);
        self.profiles.push(profile);
    }

    pub fn remove_profile(&mut self, profile_name: &str) -> bool {
        if let Some(index) = self.profiles.iter().position(|p| p == profile_name) {
            self.profiles.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_profiles(&self) -> Vec<String> {
        self.profiles.clone()
    }

    pub fn switch_profile(&mut self, profile_name: &str) -> bool {
        if let Some(index) = self.profiles.iter().position(|p| p == profile_name) {
            self.current_profile = Some(index);
            true
        } else {
            false
        }
    }

    pub fn get_current_profile(&self) -> Option<String> {
        self.current_profile.map(|index| self.profiles[index].clone())
    }
}
