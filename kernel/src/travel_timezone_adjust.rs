extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct TravelTimezoneAdjust {
    timezone_offsets: Vec<(String, i32)>,
}

impl TravelTimezoneAdjust {
    pub fn new() -> Self {
        TravelTimezoneAdjust {
            timezone_offsets: Vec::new(),
        }
    }

    pub fn add_timezone(&mut self, name: String, offset: i32) {
        self.timezone_offsets.push((name, offset));
    }

    pub fn get_offset(&self, timezone_name: &str) -> Option<i32> {
        for (name, offset) in &self.timezone_offsets {
            if name == timezone_name {
                return Some(*offset);
            }
        }
        None
    }

    pub fn list_timezones(&self) -> Vec<String> {
        self.timezone_offsets.iter().map(|(name, _)| name.clone()).collect()
    }

    pub fn adjust_time(&self, timezone_name: &str, time: i32) -> Option<i32> {
        if let Some(offset) = self.get_offset(timezone_name) {
            Some(time + offset)
        } else {
            None
        }
    }

    pub fn remove_timezone(&mut self, timezone_name: &str) {
        self.timezone_offsets.retain(|(name, _)| name != timezone_name);
    }
}
