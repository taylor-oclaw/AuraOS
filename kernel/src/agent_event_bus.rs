extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentEventBus {
    subscribers: Vec<String>,
    events: Vec<String>,
}

impl AgentEventBus {
    pub fn new() -> Self {
        AgentEventBus {
            subscribers: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn subscribe(&mut self, subscriber_id: &str) {
        if !self.subscribers.contains(&subscriber_id.to_string()) {
            self.subscribers.push(subscriber_id.to_string());
        }
    }

    pub fn unsubscribe(&mut self, subscriber_id: &str) {
        self.subscribers.retain(|s| s != subscriber_id);
    }

    pub fn publish_event(&mut self, event: &str) {
        self.events.push(event.to_string());
        self.notify_subscribers(event);
    }

    fn notify_subscribers(&self, event: &str) {
        for subscriber in &self.subscribers {
            // Simulate notification logic
            // In a real kernel module, this would involve more complex interactions
        }
    }

    pub fn get_events(&self) -> Vec<String> {
        self.events.clone()
    }
}
