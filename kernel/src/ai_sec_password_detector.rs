extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AIPasswordDetector {
    // Example field: a list of known bad passwords
    bad_passwords: Vec<String>,
}

impl AIPasswordDetector {
    pub fn new() -> Self {
        AIPasswordDetector {
            bad_passwords: Vec::new(),
        }
    }

    pub fn add_bad_password(&mut self, password: String) {
        self.bad_passwords.push(password);
    }

    pub fn remove_bad_password(&mut self, password: &str) -> bool {
        if let Some(index) = self.bad_passwords.iter().position(|p| p == password) {
            self.bad_passwords.remove(index);
            true
        } else {
            false
        }
    }

    pub fn is_password_bad(&self, password: &str) -> bool {
        self.bad_passwords.contains(&String::from(password))
    }

    pub fn list_bad_passwords(&self) -> Vec<String> {
        self.bad_passwords.clone()
    }

    pub fn clear_bad_passwords(&mut self) {
        self.bad_passwords.clear();
    }
}
