extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mdm_remote_lock_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mdm_remote_lock_exit() {
    // Cleanup logic for the module
}

pub struct RemoteLock {
    device_id: String,
    is_locked: bool,
    authorized_users: Vec<String>,
    lock_history: Vec<String>,
}

impl RemoteLock {
    pub fn new(device_id: &str) -> Self {
        RemoteLock {
            device_id: device_id.to_string(),
            is_locked: false,
            authorized_users: Vec::new(),
            lock_history: Vec::new(),
        }
    }

    pub fn lock(&mut self, user: &str) -> bool {
        if self.authorized_users.contains(&user.to_string()) {
            self.is_locked = true;
            self.lock_history.push(format!("Locked by {}", user));
            true
        } else {
            false
        }
    }

    pub fn unlock(&mut self, user: &str) -> bool {
        if self.authorized_users.contains(&user.to_string()) && self.is_locked {
            self.is_locked = false;
            self.lock_history.push(format!("Unlocked by {}", user));
            true
        } else {
            false
        }
    }

    pub fn add_authorized_user(&mut self, user: &str) -> bool {
        if !self.authorized_users.contains(&user.to_string()) {
            self.authorized_users.push(user.to_string());
            true
        } else {
            false
        }
    }

    pub fn remove_authorized_user(&mut self, user: &str) -> bool {
        if let Some(index) = self.authorized_users.iter().position(|x| x == user) {
            self.authorized_users.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_lock_history(&self) -> Vec<String> {
        self.lock_history.clone()
    }
}
