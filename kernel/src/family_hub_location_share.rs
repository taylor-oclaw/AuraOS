extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct FamilyHubLocationShare {
    locations: Vec<String>,
}

impl FamilyHubLocationShare {
    pub fn new() -> Self {
        FamilyHubLocationShare {
            locations: Vec::new(),
        }
    }

    pub fn add_location(&mut self, location: String) {
        if !self.locations.contains(&location) {
            self.locations.push(location);
        }
    }

    pub fn remove_location(&mut self, location: &str) -> bool {
        let index = self.locations.iter().position(|x| x == location);
        if let Some(i) = index {
            self.locations.remove(i);
            true
        } else {
            false
        }
    }

    pub fn get_locations(&self) -> &[String] {
        &self.locations
    }

    pub fn find_location(&self, location: &str) -> Option<&String> {
        self.locations.iter().find(|&&x| x == location)
    }

    pub fn clear_locations(&mut self) {
        self.locations.clear();
    }
}
