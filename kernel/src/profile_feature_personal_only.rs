extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ProfileFeaturePersonalOnly {
    name: String,
    age: u32,
    interests: Vec<String>,
    is_active: bool,
}

impl ProfileFeaturePersonalOnly {
    pub fn new(name: &str, age: u32) -> Self {
        ProfileFeaturePersonalOnly {
            name: String::from(name),
            age,
            interests: Vec::new(),
            is_active: true,
        }
    }

    pub fn add_interest(&mut self, interest: &str) {
        self.interests.push(String::from(interest));
    }

    pub fn remove_interest(&mut self, interest: &str) -> bool {
        if let Some(index) = self.interests.iter().position(|i| i == interest) {
            self.interests.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_interests(&self) -> Vec<String> {
        self.interests.clone()
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
}
