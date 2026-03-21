extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ProfileModeLocationTrigger {
    enabled: bool,
    locations: Vec<String>,
    current_location: Option<String>,
}

impl ProfileModeLocationTrigger {
    pub fn new() -> Self {
        ProfileModeLocationTrigger {
            enabled: false,
            locations: Vec::new(),
            current_location: None,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn add_location(&mut self, location: String) {
        if !self.locations.contains(&location) {
            self.locations.push(location);
        }
    }

    pub fn remove_location(&mut self, location: &str) {
        self.locations.retain(|l| l != location);
    }

    pub fn set_current_location(&mut self, location: String) {
        if self.enabled && self.locations.contains(&location) {
            self.current_location = Some(location);
        }
    }

    pub fn get_current_location(&self) -> Option<&String> {
        self.current_location.as_ref()
    }

    pub fn list_locations(&self) -> &Vec<String> {
        &self.locations
    }
}
