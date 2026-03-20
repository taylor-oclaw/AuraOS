extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AuraCalendar {
    events: Vec<Event>,
}

impl AuraCalendar {
    pub fn new() -> Self {
        AuraCalendar {
            events: Vec::new(),
        }
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn remove_event(&mut self, index: usize) -> Option<Event> {
        if index < self.events.len() {
            Some(self.events.remove(index))
        } else {
            None
        }
    }

    pub fn get_event(&self, index: usize) -> Option<&Event> {
        self.events.get(index)
    }

    pub fn list_events(&self) -> &[Event] {
        &self.events
    }

    pub fn count_events(&self) -> usize {
        self.events.len()
    }
}

pub struct Event {
    title: String,
    date: String,
    description: String,
}

impl Event {
    pub fn new(title: String, date: String, description: String) -> Self {
        Event {
            title,
            date,
            description,
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_date(&self) -> &str {
        &self.date
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }
}
