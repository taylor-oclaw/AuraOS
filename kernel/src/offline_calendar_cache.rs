extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct OfflineCalendarCache {
    events: Vec<Event>,
}

impl OfflineCalendarCache {
    pub fn new() -> Self {
        OfflineCalendarCache {
            events: Vec::new(),
        }
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn remove_event(&mut self, title: &str) -> bool {
        let pos = self.events.iter().position(|e| e.title == title);
        if let Some(index) = pos {
            self.events.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_events_by_date(&self, date: &Date) -> Vec<&Event> {
        self.events.iter().filter(|e| e.date == *date).collect()
    }

    pub fn get_all_events(&self) -> &[Event] {
        &self.events
    }

    pub fn clear_cache(&mut self) {
        self.events.clear();
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    title: String,
    date: Date,
    description: Option<String>,
}

impl Event {
    pub fn new(title: String, date: Date, description: Option<String>) -> Self {
        Event {
            title,
            date,
            description,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}
