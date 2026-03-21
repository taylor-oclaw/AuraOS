extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct PeopleFormalityLevel {
    name: String,
    formality_level: u8,
}

impl PeopleFormalityLevel {
    pub fn new(name: &str, formality_level: u8) -> Self {
        PeopleFormalityLevel {
            name: String::from(name),
            formality_level,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, new_name: &str) {
        self.name = String::from(new_name);
    }

    pub fn get_formality_level(&self) -> u8 {
        self.formality_level
    }

    pub fn set_formality_level(&mut self, new_level: u8) {
        if new_level > 0 && new_level <= 5 {
            self.formality_level = new_level;
        }
    }

    pub fn is_very_formal(&self) -> bool {
        self.formality_level == 5
    }

    pub fn is_casual(&self) -> bool {
        self.formality_level == 1
    }
}
