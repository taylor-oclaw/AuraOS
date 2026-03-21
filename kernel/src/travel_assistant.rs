extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct TravelAssistant {
    destinations: Vec<String>,
    current_location: String,
}

impl TravelAssistant {
    pub fn new() -> Self {
        TravelAssistant {
            destinations: Vec::new(),
            current_location: String::from("Unknown"),
        }
    }

    pub fn add_destination(&mut self, destination: &str) {
        self.destinations.push(String::from(destination));
    }

    pub fn remove_destination(&mut self, destination: &str) -> bool {
        if let Some(index) = self.destinations.iter().position(|d| d == destination) {
            self.destinations.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_destinations(&self) -> Vec<String> {
        self.destinations.clone()
    }

    pub fn set_current_location(&mut self, location: &str) {
        self.current_location = String::from(location);
    }

    pub fn get_current_location(&self) -> String {
        self.current_location.clone()
    }
}
