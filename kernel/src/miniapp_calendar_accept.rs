extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn calendar_accept_init() {
    // Initialization logic for the module
}

pub extern "C" fn calendar_accept_exit() {
    // Cleanup logic for the module
}

pub struct CalendarEvent {
    title: String,
    date: String,
    time: String,
    location: String,
    description: String,
}

impl CalendarEvent {
    pub fn new(title: &str, date: &str, time: &str, location: &str, description: &str) -> Self {
        CalendarEvent {
            title: String::from(title),
            date: String::from(date),
            time: String::from(time),
            location: String::from(location),
            description: String::from(description),
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
    }

    pub fn get_date(&self) -> &str {
        &self.date
    }

    pub fn set_date(&mut self, date: &str) {
        self.date = String::from(date);
    }

    pub fn get_time(&self) -> &str {
        &self.time
    }

    pub fn set_time(&mut self, time: &str) {
        self.time = String::from(time);
    }

    pub fn get_location(&self) -> &str {
        &self.location
    }

    pub fn set_location(&mut self, location: &str) {
        self.location = String::from(location);
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = String::from(description);
    }
}

pub struct Calendar {
    events: Vec<CalendarEvent>,
}

impl Calendar {
    pub fn new() -> Self {
        Calendar { events: Vec::new() }
    }

    pub fn add_event(&mut self, event: CalendarEvent) {
        self.events.push(event);
    }

    pub fn remove_event_by_title(&mut self, title: &str) {
        self.events.retain(|e| e.get_title() != title);
    }

    pub fn get_events_on_date(&self, date: &str) -> Vec<&CalendarEvent> {
        self.events.iter().filter(|e| e.get_date() == date).collect()
    }

    pub fn list_all_events(&self) -> Vec<&CalendarEvent> {
        self.events.iter().collect()
    }
}
