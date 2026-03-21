extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct ParentalLocationTracker {
    locations: Vec<String>,
    max_locations: usize,
}

impl ParentalLocationTracker {
    pub fn new(max_locations: usize) -> Self {
        ParentalLocationTracker {
            locations: Vec::new(),
            max_locations,
        }
    }

    pub fn add_location(&mut self, location: String) -> Result<(), &'static str> {
        if self.locations.len() >= self.max_locations {
            Err("Maximum number of locations reached")
        } else {
            self.locations.push(location);
            Ok(())
        }
    }

    pub fn get_last_location(&self) -> Option<&String> {
        self.locations.last()
    }

    pub fn remove_location(&mut self, index: usize) -> Result<String, &'static str> {
        if index < self.locations.len() {
            Ok(self.locations.remove(index))
        } else {
            Err("Index out of bounds")
        }
    }

    pub fn get_all_locations(&self) -> &[String] {
        &self.locations
    }

    pub fn clear_locations(&mut self) {
        self.locations.clear();
    }
}
