extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct EnterpriseConditionalAccess {
    users: Vec<String>,
    access_levels: Vec<u8>,
}

impl EnterpriseConditionalAccess {
    pub fn new() -> Self {
        EnterpriseConditionalAccess {
            users: Vec::new(),
            access_levels: Vec::new(),
        }
    }

    pub fn add_user(&mut self, username: &str, access_level: u8) {
        self.users.push(String::from(username));
        self.access_levels.push(access_level);
    }

    pub fn get_access_level(&self, username: &str) -> Option<u8> {
        for (i, user) in self.users.iter().enumerate() {
            if user == username {
                return Some(self.access_levels[i]);
            }
        }
        None
    }

    pub fn update_access_level(&mut self, username: &str, new_access_level: u8) -> bool {
        for i in 0..self.users.len() {
            if self.users[i] == username {
                self.access_levels[i] = new_access_level;
                return true;
            }
        }
        false
    }

    pub fn remove_user(&mut self, username: &str) -> bool {
        for i in 0..self.users.len() {
            if self.users[i] == username {
                self.users.remove(i);
                self.access_levels.remove(i);
                return true;
            }
        }
        false
    }

    pub fn list_users(&self) -> Vec<String> {
        self.users.clone()
    }
}
