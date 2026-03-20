extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct QuotaManager {
    user_quotas: Vec<(String, u32)>,
}

impl QuotaManager {
    pub fn new() -> Self {
        QuotaManager {
            user_quotas: Vec::new(),
        }
    }

    pub fn add_user(&mut self, username: &str, quota: u32) {
        let user = String::from(username);
        self.user_quotas.push((user, quota));
    }

    pub fn get_quota(&self, username: &str) -> Option<u32> {
        for (user, quota) in &self.user_quotas {
            if user == username {
                return Some(*quota);
            }
        }
        None
    }

    pub fn update_quota(&mut self, username: &str, new_quota: u32) -> bool {
        for (user, quota) in &mut self.user_quotas {
            if user == username {
                *quota = new_quota;
                return true;
            }
        }
        false
    }

    pub fn remove_user(&mut self, username: &str) -> bool {
        let pos = self.user_quotas.iter().position(|(user, _)| user == username);
        if let Some(index) = pos {
            self.user_quotas.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_users(&self) -> Vec<String> {
        self.user_quotas.iter().map(|(user, _)| user.clone()).collect()
    }
}
