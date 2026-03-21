extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn init_module() -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn cleanup_module() -> i32 {
    0
}

pub struct ContactChampion {
    name: String,
    email: String,
    phone: String,
    interests: Vec<String>,
    active: bool,
}

impl ContactChampion {
    pub fn new(name: &str, email: &str, phone: &str) -> Self {
        ContactChampion {
            name: String::from(name),
            email: String::from(email),
            phone: String::from(phone),
            interests: Vec::new(),
            active: true,
        }
    }

    pub fn add_interest(&mut self, interest: &str) {
        self.interests.push(String::from(interest));
    }

    pub fn remove_interest(&mut self, interest: &str) {
        self.interests.retain(|i| i != interest);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}
