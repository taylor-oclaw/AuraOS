extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CalendarRecurringOptimize {
    events: Vec<Event>,
}

impl CalendarRecurringOptimize {
    pub fn new() -> Self {
        CalendarRecurringOptimize {
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

    pub fn get_events_by_date(&self, date: &str) -> Vec<&Event> {
        self.events.iter().filter(|e| e.date == *date).collect()
    }

    pub fn find_event_by_name(&self, name: &str) -> Option<&Event> {
        self.events.iter().find(|e| e.name == *name)
    }

    pub fn list_all_events(&self) -> Vec<&Event> {
        self.events.iter().collect()
    }
}

pub struct Event {
    name: String,
    date: String,
    // Add more fields as needed
}

impl Event {
    pub fn new(name: String, date: String) -> Self {
        Event { name, date }
    }
}
