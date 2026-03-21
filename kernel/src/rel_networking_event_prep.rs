extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct NetworkEventPrep {
    events: Vec<String>,
}

impl NetworkEventPrep {
    pub fn new() -> Self {
        NetworkEventPrep {
            events: Vec::new(),
        }
    }

    pub fn add_event(&mut self, event: String) {
        self.events.push(event);
    }

    pub fn remove_event(&mut self, index: usize) -> Option<String> {
        if index < self.events.len() {
            Some(self.events.remove(index))
        } else {
            None
        }
    }

    pub fn get_event(&self, index: usize) -> Option<&String> {
        self.events.get(index)
    }

    pub fn list_events(&self) -> &[String] {
        &self.events
    }

    pub fn clear_events(&mut self) {
        self.events.clear();
    }
}
