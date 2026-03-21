extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct NotificationSummaryDigest {
    notifications: Vec<String>,
}

impl NotificationSummaryDigest {
    pub fn new() -> Self {
        NotificationSummaryDigest {
            notifications: Vec::new(),
        }
    }

    pub fn add_notification(&mut self, notification: String) {
        self.notifications.push(notification);
    }

    pub fn remove_notification(&mut self, index: usize) -> Option<String> {
        if index < self.notifications.len() {
            Some(self.notifications.remove(index))
        } else {
            None
        }
    }

    pub fn get_notifications_count(&self) -> usize {
        self.notifications.len()
    }

    pub fn get_all_notifications(&self) -> Vec<String> {
        self.notifications.clone()
    }

    pub fn clear_notifications(&mut self) {
        self.notifications.clear();
    }
}
