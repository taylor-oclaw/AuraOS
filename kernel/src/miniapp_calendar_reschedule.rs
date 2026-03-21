extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CalendarEvent {
    title: String,
    start_time: u64, // Unix timestamp in seconds
    end_time: u64,   // Unix timestamp in seconds
}

impl CalendarEvent {
    pub fn new(title: &str, start_time: u64, end_time: u64) -> Self {
        CalendarEvent {
            title: String::from(title),
            start_time,
            end_time,
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn set_title(&mut self, new_title: &str) {
        self.title = String::from(new_title);
    }

    pub fn get_start_time(&self) -> u64 {
        self.start_time
    }

    pub fn set_start_time(&mut self, new_start_time: u64) {
        self.start_time = new_start_time;
    }

    pub fn get_end_time(&self) -> u64 {
        self.end_time
    }

    pub fn set_end_time(&mut self, new_end_time: u64) {
        self.end_time = new_end_time;
    }

    pub fn is_overlapping(&self, other: &CalendarEvent) -> bool {
        self.start_time < other.end_time && other.start_time < self.end_time
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
        self.events.retain(|e| e.title != title);
    }

    pub fn get_events(&self) -> &[CalendarEvent] {
        &self.events
    }

    pub fn find_overlapping_events(&self, event: &CalendarEvent) -> Vec<&CalendarEvent> {
        self.events.iter().filter(|&e| e.is_overlapping(event)).collect()
    }
}
