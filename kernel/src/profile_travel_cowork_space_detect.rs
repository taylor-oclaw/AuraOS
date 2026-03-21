extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct ProfileTravelCoworkSpaceDetect {
    profiles: Vec<String>,
    cowork_spaces: Vec<String>,
    travel_destinations: Vec<String>,
}

impl ProfileTravelCoworkSpaceDetect {
    pub fn new() -> Self {
        ProfileTravelCoworkSpaceDetect {
            profiles: Vec::new(),
            cowork_spaces: Vec::new(),
            travel_destinations: Vec::new(),
        }
    }

    pub fn add_profile(&mut self, profile: String) {
        self.profiles.push(profile);
    }

    pub fn remove_profile(&mut self, profile: &str) -> bool {
        if let Some(index) = self.profiles.iter().position(|p| p == profile) {
            self.profiles.remove(index);
            true
        } else {
            false
        }
    }

    pub fn add_cowork_space(&mut self, cowork_space: String) {
        self.cowork_spaces.push(cowork_space);
    }

    pub fn remove_cowork_space(&mut self, cowork_space: &str) -> bool {
        if let Some(index) = self.cowork_spaces.iter().position(|c| c == cowork_space) {
            self.cowork_spaces.remove(index);
            true
        } else {
            false
        }
    }

    pub fn add_travel_destination(&mut self, destination: String) {
        self.travel_destinations.push(destination);
    }

    pub fn remove_travel_destination(&mut self, destination: &str) -> bool {
        if let Some(index) = self.travel_destinations.iter().position(|d| d == destination) {
            self.travel_destinations.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_profiles(&self) -> Vec<String> {
        self.profiles.clone()
    }

    pub fn list_cowork_spaces(&self) -> Vec<String> {
        self.cowork_spaces.clone()
    }

    pub fn list_travel_destinations(&self) -> Vec<String> {
        self.travel_destinations.clone()
    }
}
