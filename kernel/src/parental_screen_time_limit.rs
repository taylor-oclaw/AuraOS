extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ParentalScreenTimeLimit {
    user_limits: Vec<(String, u32)>, // (username, time_limit_in_minutes)
}

impl ParentalScreenTimeLimit {
    pub fn new() -> Self {
        ParentalScreenTimeLimit {
            user_limits: Vec::new(),
        }
    }

    pub fn add_user_limit(&mut self, username: String, limit: u32) {
        self.user_limits.push((username, limit));
    }

    pub fn get_user_limit(&self, username: &str) -> Option<u32> {
        for (user, limit) in &self.user_limits {
            if user == username {
                return Some(*limit);
            }
        }
        None
    }

    pub fn remove_user_limit(&mut self, username: &str) {
        self.user_limits.retain(|(user, _)| user != username);
    }

    pub fn update_user_limit(&mut self, username: &str, new_limit: u32) -> bool {
        for (user, limit) in &mut self.user_limits {
            if user == username {
                *limit = new_limit;
                return true;
            }
        }
        false
    }

    pub fn list_user_limits(&self) -> Vec<(String, u32)> {
        self.user_limits.clone()
    }
}
