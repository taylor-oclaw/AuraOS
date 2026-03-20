extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialize the module if needed
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Clean up the module if needed
}

pub struct AuraAccessibilityMgr {
    users: Vec<String>,
    active_user: Option<usize>,
}

impl AuraAccessibilityMgr {
    pub fn new() -> Self {
        AuraAccessibilityMgr {
            users: Vec::new(),
            active_user: None,
        }
    }

    pub fn add_user(&mut self, username: &str) {
        let user = String::from(username);
        self.users.push(user);
    }

    pub fn remove_user(&mut self, username: &str) -> bool {
        if let Some(index) = self.users.iter().position(|u| u == username) {
            self.users.remove(index);
            true
        } else {
            false
        }
    }

    pub fn set_active_user(&mut self, username: &str) -> bool {
        if let Some(index) = self.users.iter().position(|u| u == username) {
            self.active_user = Some(index);
            true
        } else {
            false
        }
    }

    pub fn get_active_user(&self) -> Option<&String> {
        self.active_user.map(|index| &self.users[index])
    }

    pub fn list_users(&self) -> Vec<&String> {
        self.users.iter().collect()
    }
}
