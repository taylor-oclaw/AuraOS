extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct NotificationMorningDigest {
    notifications: Vec<String>,
}

impl NotificationMorningDigest {
    pub fn new() -> Self {
        NotificationMorningDigest {
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

    pub fn get_notifications(&self) -> &Vec<String> {
        &self.notifications
    }

    pub fn clear_notifications(&mut self) {
        self.notifications.clear();
    }

    pub fn has_notification(&self, notification: &str) -> bool {
        self.notifications.iter().any(|n| n == notification)
    }
}
