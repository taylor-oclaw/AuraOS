extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PriorityCalendar {
    events: Vec<(u32, String)>, // (priority, event)
}

impl PriorityCalendar {
    pub fn new() -> Self {
        PriorityCalendar {
            events: Vec::new(),
        }
    }

    pub fn add_event(&mut self, priority: u32, event: String) {
        self.events.push((priority, event));
        self.events.sort_by(|a, b| b.0.cmp(&a.0)); // Sort by priority descending
    }

    pub fn remove_event(&mut self, event_name: &str) -> bool {
        let pos = self.events.iter().position(|&(_, ref name)| name == event_name);
        if let Some(index) = pos {
            self.events.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_top_event(&self) -> Option<&String> {
        self.events.first().map(|(_, event)| event)
    }

    pub fn list_events(&self) -> Vec<String> {
        self.events.iter().map(|(_, event)| event.clone()).collect()
    }

    pub fn clear_events(&mut self) {
        self.events.clear();
    }
}
