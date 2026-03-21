extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct NotificationWeeklyDigest {
    notifications: Vec<String>,
}

impl NotificationWeeklyDigest {
    pub fn new() -> Self {
        NotificationWeeklyDigest {
            notifications: Vec::new(),
        }
    }

    pub fn add_notification(&mut self, notification: String) {
        self.notifications.push(notification);
    }

    pub fn get_notifications_count(&self) -> usize {
        self.notifications.len()
    }

    pub fn clear_notifications(&mut self) {
        self.notifications.clear();
    }

    pub fn get_all_notifications(&self) -> Vec<String> {
        self.notifications.clone()
    }

    pub fn has_notification_with_keyword(&self, keyword: &str) -> bool {
        for notification in &self.notifications {
            if notification.contains(keyword) {
                return true;
            }
        }
        false
    }
}
