extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AnalyticsScreenTime {
    user_id: String,
    screen_time_data: Vec<(u64, u32)>, // (timestamp, screen_time_in_seconds)
}

impl AnalyticsScreenTime {
    pub fn new(user_id: &str) -> Self {
        AnalyticsScreenTime {
            user_id: String::from(user_id),
            screen_time_data: Vec::new(),
        }
    }

    pub fn add_screen_time(&mut self, timestamp: u64, screen_time_in_seconds: u32) {
        self.screen_time_data.push((timestamp, screen_time_in_seconds));
    }

    pub fn total_screen_time(&self) -> u32 {
        self.screen_time_data.iter().map(|&(_, time)| time).sum()
    }

    pub fn average_screen_time(&self) -> f32 {
        if self.screen_time_data.is_empty() {
            0.0
        } else {
            self.total_screen_time() as f32 / self.screen_time_data.len() as f32
        }
    }

    pub fn get_screen_time_at_timestamp(&self, timestamp: u64) -> Option<u32> {
        self.screen_time_data.iter().find(|&&(t, _)| t == timestamp).map(|&(_, time)| time)
    }

    pub fn clear_data(&mut self) {
        self.screen_time_data.clear();
    }
}
