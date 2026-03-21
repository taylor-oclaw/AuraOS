extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PredictCalendarContext {
    events: Vec<String>,
}

impl PredictCalendarContext {
    pub fn new() -> Self {
        PredictCalendarContext {
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

    pub fn get_events(&self) -> &Vec<String> {
        &self.events
    }

    pub fn find_event(&self, event_name: &str) -> Option<&String> {
        self.events.iter().find(|&&event| event == event_name)
    }

    pub fn clear_events(&mut self) {
        self.events.clear();
    }
}
