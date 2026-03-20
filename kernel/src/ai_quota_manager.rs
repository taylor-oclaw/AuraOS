extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AiQuotaManager {
    user_quotas: Vec<(String, u32)>,
}

impl AiQuotaManager {
    pub fn new() -> Self {
        AiQuotaManager {
            user_quotas: Vec::new(),
        }
    }

    pub fn add_user(&mut self, username: String, quota: u32) {
        self.user_quotas.push((username, quota));
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
        let pos = self.user_quotas.iter().position(|&(ref user, _)| user == username);
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
