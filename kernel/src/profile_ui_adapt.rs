extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ProfileUIAdapt {
    user_name: String,
    age: u32,
    email: String,
    interests: Vec<String>,
    is_active: bool,
}

impl ProfileUIAdapt {
    pub fn new(user_name: &str, age: u32, email: &str) -> Self {
        ProfileUIAdapt {
            user_name: String::from(user_name),
            age,
            email: String::from(email),
            interests: Vec::new(),
            is_active: true,
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

    pub fn get_user_name(&self) -> &str {
        &self.user_name
    }

    pub fn set_age(&mut self, age: u32) {
        self.age = age;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
}
