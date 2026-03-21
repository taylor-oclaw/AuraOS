extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct ProfileAccount {
    username: String,
    email: String,
    age: u8,
    interests: Vec<String>,
    active: bool,
}

impl ProfileAccount {
    pub fn new(username: &str, email: &str, age: u8) -> Self {
        ProfileAccount {
            username: String::from(username),
            email: String::from(email),
            age,
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

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}
