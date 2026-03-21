extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PhotoEventDetect {
    events: Vec<String>,
}

impl PhotoEventDetect {
    pub fn new() -> Self {
        PhotoEventDetect {
            events: Vec::new(),
        }
    }

    pub fn add_event(&mut self, event: String) {
        self.events.push(event);
    }

    pub fn get_events(&self) -> &Vec<String> {
        &self.events
    }

    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    pub fn has_event(&self, event: &str) -> bool {
        self.events.iter().any(|e| e == event)
    }

    pub fn count_events(&self) -> usize {
        self.events.len()
    }
}
