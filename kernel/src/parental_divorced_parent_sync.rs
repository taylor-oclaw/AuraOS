extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ParentalDivorcedParentSync {
    parent1_name: String,
    parent2_name: String,
    children_names: Vec<String>,
    custody_schedule: Vec<(u8, u8)>, // (start_hour, end_hour)
    communication_log: Vec<String>,
}

impl ParentalDivorcedParentSync {
    pub fn new(parent1_name: &str, parent2_name: &str) -> Self {
        ParentalDivorcedParentSync {
            parent1_name: String::from(parent1_name),
            parent2_name: String::from(parent2_name),
            children_names: Vec::new(),
            custody_schedule: Vec::new(),
            communication_log: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child_name: &str) {
        self.children_names.push(String::from(child_name));
    }

    pub fn set_custody_schedule(&mut self, schedule: Vec<(u8, u8)>) {
        self.custody_schedule = schedule;
    }

    pub fn log_communication(&mut self, message: &str) {
        self.communication_log.push(String::from(message));
    }

    pub fn get_caregiver_for_time(&self, hour: u8) -> Option<&str> {
        for (start_hour, end_hour) in &self.custody_schedule {
            if hour >= *start_hour && hour < *end_hour {
                return Some(if hour % 2 == 0 { &self.parent1_name } else { &self.parent2_name });
            }
        }
        None
    }

    pub fn get_communication_log(&self) -> &[String] {
        &self.communication_log
    }
}
