extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AuraTimeTracker {
    events: Vec<(String, u64)>,
}

impl AuraTimeTracker {
    pub fn new() -> Self {
        AuraTimeTracker {
            events: Vec::new(),
        }
    }

    pub fn add_event(&mut self, event_name: String, timestamp: u64) {
        self.events.push((event_name, timestamp));
    }

    pub fn get_events(&self) -> &Vec<(String, u64)> {
        &self.events
    }

    pub fn get_latest_event(&self) -> Option<&(String, u64)> {
        self.events.last()
    }

    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    pub fn count_events(&self) -> usize {
        self.events.len()
    }
}
