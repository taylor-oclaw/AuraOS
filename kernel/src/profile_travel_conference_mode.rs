extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct ProfileTravelClientVisitMode {
    client_id: u32,
    visit_count: u32,
    locations_visited: Vec<String>,
    last_visit_time: u64, // Unix timestamp
    is_active: bool,
}

impl ProfileTravelClientVisitMode {
    pub fn new(client_id: u32) -> Self {
        ProfileTravelClientVisitMode {
            client_id,
            visit_count: 0,
            locations_visited: Vec::new(),
            last_visit_time: 0,
            is_active: true,
        }
    }

    pub fn add_location(&mut self, location: String) {
        if !self.locations_visited.contains(&location) {
            self.locations_visited.push(location);
            self.visit_count += 1;
            self.last_visit_time = current_timestamp();
        }
    }

    pub fn get_client_id(&self) -> u32 {
        self.client_id
    }

    pub fn get_visit_count(&self) -> u32 {
        self.visit_count
    }

    pub fn get_locations_visited(&self) -> &Vec<String> {
        &self.locations_visited
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn is_client_active(&self) -> bool {
        self.is_active
    }
}

fn current_timestamp() -> u64 {
    // Placeholder for actual timestamp retrieval logic
    1633072800 // Example Unix timestamp
}
