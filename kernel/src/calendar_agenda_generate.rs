extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CalendarAgenda {
    events: Vec<Event>,
}

impl CalendarAgenda {
    pub fn new() -> Self {
        CalendarAgenda { events: Vec::new() }
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

    pub fn find_events_by_date(&self, date: &str) -> Vec<&Event> {
        self.events.iter().filter(|e| e.date == date).collect()
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
        Event { title, date, time, location, description }
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
