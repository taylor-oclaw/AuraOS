extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CalendarEvent {
    title: String,
    description: String,
    start_time: u64, // Unix timestamp in seconds
    end_time: u64,   // Unix timestamp in seconds
    location: String,
}

impl CalendarEvent {
    pub fn new(title: &str, description: &str, start_time: u64, end_time: u64, location: &str) -> Self {
        CalendarEvent {
            title: String::from(title),
            description: String::from(description),
            start_time,
            end_time,
            location: String::from(location),
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = String::from(description);
    }

    pub fn get_start_time(&self) -> u64 {
        self.start_time
    }

    pub fn set_start_time(&mut self, start_time: u64) {
        self.start_time = start_time;
    }

    pub fn get_end_time(&self) -> u64 {
        self.end_time
    }

    pub fn set_end_time(&mut self, end_time: u64) {
        self.end_time = end_time;
    }

    pub fn get_location(&self) -> &str {
        &self.location
    }

    pub fn set_location(&mut self, location: &str) {
        self.location = String::from(location);
    }

    pub fn is_ongoing(&self, current_time: u64) -> bool {
        current_time >= self.start_time && current_time <= self.end_time
    }

    pub fn duration_in_seconds(&self) -> u64 {
        if self.end_time > self.start_time {
            self.end_time - self.start_time
        } else {
            0
        }
    }
}
