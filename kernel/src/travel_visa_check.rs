extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct TravelVisaCheck {
    visa_list: Vec<String>,
}

impl TravelVisaCheck {
    pub fn new() -> Self {
        TravelVisaCheck {
            visa_list: Vec::new(),
        }
    }

    pub fn add_visa(&mut self, visa_id: &str) {
        self.visa_list.push(String::from(visa_id));
    }

    pub fn remove_visa(&mut self, visa_id: &str) -> bool {
        if let Some(index) = self.visa_list.iter().position(|v| v == visa_id) {
            self.visa_list.remove(index);
            true
        } else {
            false
        }
    }

    pub fn check_visa(&self, visa_id: &str) -> bool {
        self.visa_list.contains(&String::from(visa_id))
    }

    pub fn list_visas(&self) -> Vec<String> {
        self.visa_list.clone()
    }

    pub fn count_visas(&self) -> usize {
        self.visa_list.len()
    }
}
