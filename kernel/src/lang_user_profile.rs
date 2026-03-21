extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct UserProfile {
    username: String,
    age: u32,
    email: String,
    interests: Vec<String>,
    active: bool,
}

impl UserProfile {
    pub fn new(username: &str, age: u32, email: &str) -> Self {
        UserProfile {
            username: String::from(username),
            age,
            email: String::from(email),
            interests: Vec::new(),
            active: true,
        }
    }

    pub fn add_interest(&mut self, interest: &str) {
        self.interests.push(String::from(interest));
    }

    pub fn remove_interest(&mut self, interest: &str) -> bool {
        if let Some(pos) = self.interests.iter().position(|i| i == interest) {
            self.interests.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}
