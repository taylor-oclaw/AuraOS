extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_notification_timing_optimal_init() {
    // Initialization code for the module
}

pub extern "C" fn rust_notification_timing_optimal_exit() {
    // Cleanup code for the module
}

pub struct NotificationTimingOptimal {
    notifications: Vec<(String, u64)>, // (notification_message, timestamp)
}

impl NotificationTimingOptimal {
    pub fn new() -> Self {
        NotificationTimingOptimal {
            notifications: Vec::new(),
        }
    }

    pub fn add_notification(&mut self, message: String, timestamp: u64) {
        self.notifications.push((message, timestamp));
    }

    pub fn get_notifications(&self) -> &Vec<(String, u64)> {
        &self.notifications
    }

    pub fn remove_notification(&mut self, index: usize) -> Option<(String, u64)> {
        if index < self.notifications.len() {
            Some(self.notifications.remove(index))
        } else {
            None
        }
    }

    pub fn get_next_notification_time(&self) -> Option<u64> {
        self.notifications.iter().map(|(_, &timestamp)| timestamp).min()
    }

    pub fn clear_notifications(&mut self) {
        self.notifications.clear();
    }
}
