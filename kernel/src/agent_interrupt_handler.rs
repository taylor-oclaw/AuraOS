extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentInterruptHandler {
    // Example fields for demonstration purposes
    name: String,
    active: bool,
    events: Vec<String>,
}

impl AgentInterruptHandler {
    pub fn new(name: &str) -> Self {
        AgentInterruptHandler {
            name: String::from(name),
            active: false,
            events: Vec::new(),
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn add_event(&mut self, event: &str) {
        self.events.push(String::from(event));
    }

    pub fn get_events(&self) -> &[String] {
        &self.events
    }
}

// Example usage within the kernel module (not part of the struct impl)
fn main() {
    let mut handler = AgentInterruptHandler::new("AI-Agent");
    handler.activate();
    handler.add_event("Event 1");
    handler.add_event("Event 2");

    if handler.is_active() {
        for event in handler.get_events() {
            // Simulate logging or processing the event
        }
    }

    handler.deactivate();
}
