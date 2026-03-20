extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct NotificationCenter {
    subscribers: Vec<String>,
    notifications: Vec<String>,
}

impl NotificationCenter {
    pub fn new() -> Self {
        NotificationCenter {
            subscribers: Vec::new(),
            notifications: Vec::new(),
        }
    }

    pub fn subscribe(&mut self, subscriber: String) {
        if !self.subscribers.contains(&subscriber) {
            self.subscribers.push(subscriber);
        }
    }

    pub fn unsubscribe(&mut self, subscriber: &str) {
        self.subscribers.retain(|s| s != subscriber);
    }

    pub fn notify_all(&mut self, notification: String) {
        self.notifications.push(notification.clone());
        for _ in &self.subscribers {
            // Simulate sending a notification
            // In a real kernel module, this would involve more complex logic
        }
    }

    pub fn get_notifications(&self) -> Vec<String> {
        self.notifications.clone()
    }

    pub fn clear_notifications(&mut self) {
        self.notifications.clear();
    }
}
