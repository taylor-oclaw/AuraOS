extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct AgentPubSub {
    subscribers: Vec<String>,
    messages: Vec<String>,
}

impl AgentPubSub {
    pub fn new() -> Self {
        AgentPubSub {
            subscribers: Vec::new(),
            messages: Vec::new(),
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

    pub fn publish(&mut self, message: &str) {
        self.messages.push(message.to_string());
    }

    pub fn get_messages(&self) -> Vec<String> {
        self.messages.clone()
    }

    pub fn clear_messages(&mut self) {
        self.messages.clear();
    }
}

#[no_mangle]
pub extern "C" fn agent_pub_sub_init() {
    // Initialize the module
    let mut pub_sub = AgentPubSub::new();
    pub_sub.subscribe("subscriber1");
    pub_sub.publish("Hello, world!");
    println!("Messages: {:?}", pub_sub.get_messages());
    pub_sub.clear_messages();
}
