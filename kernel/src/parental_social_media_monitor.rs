extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ParentalSocialMediaMonitor {
    blocked_keywords: Vec<String>,
    allowed_users: Vec<String>,
    user_activity_log: Vec<String>,
}

impl ParentalSocialMediaMonitor {
    pub fn new() -> Self {
        ParentalSocialMediaMonitor {
            blocked_keywords: Vec::new(),
            allowed_users: Vec::new(),
            user_activity_log: Vec::new(),
        }
    }

    pub fn add_blocked_keyword(&mut self, keyword: String) {
        if !self.blocked_keywords.contains(&keyword) {
            self.blocked_keywords.push(keyword);
        }
    }

    pub fn remove_blocked_keyword(&mut self, keyword: &str) {
        self.blocked_keywords.retain(|k| k != keyword);
    }

    pub fn add_allowed_user(&mut self, user: String) {
        if !self.allowed_users.contains(&user) {
            self.allowed_users.push(user);
        }
    }

    pub fn remove_allowed_user(&mut self, user: &str) {
        self.allowed_users.retain(|u| u != user);
    }

    pub fn log_activity(&mut self, activity: String) {
        self.user_activity_log.push(activity);
    }

    pub fn check_activity(&self, activity: &str) -> bool {
        for keyword in &self.blocked_keywords {
            if activity.contains(keyword) {
                return false;
            }
        }
        true
    }

    pub fn get_user_activity_log(&self) -> &Vec<String> {
        &self.user_activity_log
    }
}
