extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct NotificationChannelRoute {
    subscribers: Vec<String>,
}

impl NotificationChannelRoute {
    pub fn new() -> Self {
        NotificationChannelRoute {
            subscribers: Vec::new(),
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

    pub fn notify_all(&self, message: &str) -> Vec<String> {
        self.subscribers.iter().map(|subscriber| format!("{} received: {}", subscriber, message)).collect()
    }

    pub fn get_subscriber_count(&self) -> usize {
        self.subscribers.len()
    }

    pub fn list_subscribers(&self) -> Vec<&String> {
        self.subscribers.iter().collect()
    }
}
