extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct CoworkerAvailability {
    coworkers: Vec<String>,
    available: bool,
}

impl CoworkerAvailability {
    pub fn new(coworkers: Vec<String>) -> Self {
        CoworkerAvailability {
            coworkers,
            available: true,
        }
    }

    pub fn add_coworker(&mut self, name: String) {
        self.coworkers.push(name);
    }

    pub fn remove_coworker(&mut self, name: &str) -> bool {
        if let Some(index) = self.coworkers.iter().position(|x| x == name) {
            self.coworkers.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_coworkers(&self) -> Vec<String> {
        self.coworkers.clone()
    }

    pub fn set_availability(&mut self, available: bool) {
        self.available = available;
    }

    pub fn is_available(&self) -> bool {
        self.available
    }
}
