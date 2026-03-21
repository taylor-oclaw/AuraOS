extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ProfileModeCalendarTrigger {
    events: Vec<(u32, String)>, // (timestamp, event_description)
}

impl ProfileModeCalendarTrigger {
    pub fn new() -> Self {
        ProfileModeCalendarTrigger {
            events: Vec::new(),
        }
    }

    pub fn add_event(&mut self, timestamp: u32, description: String) {
        self.events.push((timestamp, description));
    }

    pub fn remove_event(&mut self, timestamp: u32) -> Option<String> {
        let index = self.events.iter().position(|&(t, _)| t == timestamp);
        match index {
            Some(i) => Some(self.events.remove(i).1),
            None => None,
        }
    }

    pub fn get_event(&self, timestamp: u32) -> Option<&String> {
        self.events.iter().find_map(|&(t, ref desc)| if t == timestamp { Some(desc) } else { None })
    }

    pub fn list_events(&self) -> &Vec<(u32, String)> {
        &self.events
    }

    pub fn clear_events(&mut self) {
        self.events.clear();
    }
}
