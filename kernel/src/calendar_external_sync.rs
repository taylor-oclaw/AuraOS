extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CalendarExternalSync {
    events: Vec<Event>,
}

impl CalendarExternalSync {
    pub fn new() -> Self {
        CalendarExternalSync {
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

    pub fn get_events(&self) -> &[Event] {
        &self.events
    }

    pub fn find_event_by_title(&self, title: &str) -> Option<&Event> {
        self.events.iter().find(|e| e.title == title)
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
    title: String,
    date: String,
    time: String,
    location: String,
    description: String,
}

impl Event {
    pub fn new(title: String, date: String, time: String, location: String, description: String) -> Self {
        Event {
            title,
            date,
            time,
            location,
            description,
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title
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
