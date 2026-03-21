extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn calendar_prep_time_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn calendar_prep_time_exit() {
    // Cleanup logic for the module
}

pub struct CalendarPrepTime {
    events: Vec<Event>,
}

impl CalendarPrepTime {
    pub fn new() -> Self {
        CalendarPrepTime {
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
}

#[derive(Debug)]
pub struct Event {
    title: String,
    date: Date,
    duration: u32, // in minutes
}

impl Event {
    pub fn new(title: String, date: Date, duration: u32) -> Self {
        Event {
            title,
            date,
            duration,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

impl Date {
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Date { year, month, day }
    }

    pub fn is_before(&self, other: &Date) -> bool {
        self.year < other.year || 
        (self.year == other.year && self.month < other.month) ||
        (self.year == other.year && self.month == other.month && self.day < other.day)
    }

    pub fn is_after(&self, other: &Date) -> bool {
        self.year > other.year || 
        (self.year == other.year && self.month > other.month) ||
        (self.year == other.year && self.month == other.month && self.day > other.day)
    }
}
