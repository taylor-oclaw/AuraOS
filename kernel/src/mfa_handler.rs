extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct MfaHandler {
    users: Vec<String>,
    codes: Vec<u32>,
}

impl MfaHandler {
    pub fn new() -> Self {
        MfaHandler {
            users: Vec::new(),
            codes: Vec::new(),
        }
    }

    pub fn add_user(&mut self, username: &str) {
        let user = String::from(username);
        if !self.users.contains(&user) {
            self.users.push(user);
            self.codes.push(0); // Initialize with a dummy code
        }
    }

    pub fn remove_user(&mut self, username: &str) {
        let index = self.users.iter().position(|u| u == username);
        if let Some(idx) = index {
            self.users.remove(idx);
            self.codes.remove(idx);
        }
    }

    pub fn set_code(&mut self, username: &str, code: u32) -> bool {
        if let Some(index) = self.users.iter().position(|u| u == username) {
            self.codes[index] = code;
            true
        } else {
            false
        }
    }

    pub fn get_code(&self, username: &str) -> Option<u32> {
        self.users.iter().position(|u| u == username).map(|idx| self.codes[idx])
    }

    pub fn authenticate(&self, username: &str, code: u32) -> bool {
        if let Some(index) = self.users.iter().position(|u| u == username) {
            self.codes[index] == code
        } else {
            false
        }
    }
}
