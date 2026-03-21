extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct NotificationBundle {
    notifications: Vec<String>,
}

impl NotificationBundle {
    pub fn new() -> Self {
        NotificationBundle {
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

    pub fn get_notifications(&self) -> &[String] {
        &self.notifications
    }

    pub fn clear_notifications(&mut self) {
        self.notifications.clear();
    }

    pub fn has_notification(&self, notification: &str) -> bool {
        self.notifications.iter().any(|n| n == notification)
    }
}
