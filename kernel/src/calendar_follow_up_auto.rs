extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CalendarFollowUpAuto {
    events: Vec<Event>,
}

impl CalendarFollowUpAuto {
    pub fn new() -> Self {
        CalendarFollowUpAuto {
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

    pub fn get_events(&self) -> &Vec<Event> {
        &self.events
    }

    pub fn find_event_by_name(&self, name: &str) -> Option<&Event> {
        self.events.iter().find(|e| e.name == name)
    }

    pub fn update_event(&mut self, index: usize, event: Event) -> bool {
        if index < self.events.len() {
            self.events[index] = event;
            true
        } else {
            false
        }
    }
}

pub struct Event {
    name: String,
    date: String,
    time: String,
    location: String,
    description: String,
}

impl Event {
    pub fn new(name: String, date: String, time: String, location: String, description: String) -> Self {
        Event {
            name,
            date,
            time,
            location,
            description,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_date(&self) -> &str {
        &self.date
    }

    pub fn get_time(&self) -> &str {
        &self.time
    }

    pub fn get_location(&self) -> &str {
        &self.location
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }
}
