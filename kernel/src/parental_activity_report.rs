extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ParentalActivityReport {
    user_id: u32,
    activities: Vec<String>,
}

impl ParentalActivityReport {
    pub fn new(user_id: u32) -> Self {
        ParentalActivityReport {
            user_id,
            activities: Vec::new(),
        }
    }

    pub fn add_activity(&mut self, activity: &str) {
        self.activities.push(activity.to_string());
    }

    pub fn get_user_id(&self) -> u32 {
        self.user_id
    }

    pub fn get_activities(&self) -> &[String] {
        &self.activities
    }

    pub fn clear_activities(&mut self) {
        self.activities.clear();
    }

    pub fn has_activity(&self, activity: &str) -> bool {
        self.activities.contains(&activity.to_string())
    }
}
