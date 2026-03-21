extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PeopleBestTimeContact {
    name: String,
    best_time: u32, // Time in minutes since midnight
}

impl PeopleBestTimeContact {
    pub fn new(name: &str, best_time: u32) -> Self {
        PeopleBestTimeContact {
            name: String::from(name),
            best_time,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, new_name: &str) {
        self.name = String::from(new_name);
    }

    pub fn get_best_time(&self) -> u32 {
        self.best_time
    }

    pub fn set_best_time(&mut self, new_best_time: u32) {
        self.best_time = new_best_time;
    }

    pub fn is_best_time_after_midnight(&self) -> bool {
        self.best_time >= 720 // 12 PM in minutes since midnight
    }
}
