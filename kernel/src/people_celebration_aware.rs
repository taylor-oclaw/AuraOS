extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn people_celebration_aware_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn people_celebration_aware_exit() {
    // Cleanup logic for the module
}

pub struct PeopleCelebrationAware {
    names: Vec<String>,
    celebrations: Vec<String>,
}

impl PeopleCelebrationAware {
    pub fn new() -> Self {
        PeopleCelebrationAware {
            names: Vec::new(),
            celebrations: Vec::new(),
        }
    }

    pub fn add_person(&mut self, name: String) {
        self.names.push(name);
    }

    pub fn remove_person(&mut self, name: &str) -> bool {
        if let Some(index) = self.names.iter().position(|n| n == name) {
            self.names.remove(index);
            true
        } else {
            false
        }
    }

    pub fn add_celebration(&mut self, celebration: String) {
        self.celebrations.push(celebration);
    }

    pub fn remove_celebration(&mut self, celebration: &str) -> bool {
        if let Some(index) = self.celebrations.iter().position(|c| c == celebration) {
            self.celebrations.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_people(&self) -> Vec<String> {
        self.names.clone()
    }

    pub fn list_celebrations(&self) -> Vec<String> {
        self.celebrations.clone()
    }
}
