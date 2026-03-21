extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AnalyticsAppUsage {
    app_name: String,
    usage_count: u32,
    last_used_time: u64,
    total_usage_time: u64,
    user_ids: Vec<u32>,
}

impl AnalyticsAppUsage {
    pub fn new(app_name: &str) -> Self {
        AnalyticsAppUsage {
            app_name: String::from(app_name),
            usage_count: 0,
            last_used_time: 0,
            total_usage_time: 0,
            user_ids: Vec::new(),
        }
    }

    pub fn increment_usage(&mut self, current_time: u64) {
        self.usage_count += 1;
        self.last_used_time = current_time;
    }

    pub fn add_user_id(&mut self, user_id: u32) {
        if !self.user_ids.contains(&user_id) {
            self.user_ids.push(user_id);
        }
    }

    pub fn get_app_name(&self) -> &str {
        &self.app_name
    }

    pub fn get_usage_count(&self) -> u32 {
        self.usage_count
    }

    pub fn get_last_used_time(&self) -> u64 {
        self.last_used_time
    }

    pub fn get_total_usage_time(&self) -> u64 {
        self.total_usage_time
    }

    pub fn get_user_ids(&self) -> &Vec<u32> {
        &self.user_ids
    }
}
