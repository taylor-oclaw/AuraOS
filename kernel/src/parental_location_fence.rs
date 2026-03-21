extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ParentalLocationFence {
    // Define the fields for your module here
    fences: Vec<String>,
}

impl ParentalLocationFence {
    pub fn new() -> Self {
        ParentalLocationFence {
            fences: Vec::new(),
        }
    }

    pub fn add_fence(&mut self, location: &str) {
        self.fences.push(location.to_string());
    }

    pub fn remove_fence(&mut self, location: &str) {
        if let Some(index) = self.fences.iter().position(|f| f == location) {
            self.fences.remove(index);
        }
    }

    pub fn is_location_allowed(&self, location: &str) -> bool {
        !self.fences.contains(&location.to_string())
    }

    pub fn list_fences(&self) -> Vec<String> {
        self.fences.clone()
    }

    pub fn clear_all_fences(&mut self) {
        self.fences.clear();
    }
}
