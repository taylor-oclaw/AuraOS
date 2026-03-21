extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PeopleCalendar {
    events: Vec<Event>,
}

impl PeopleCalendar {
    pub fn new() -> Self {
        PeopleCalendar { events: Vec::new() }
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
        self.events.iter().filter(|e| &e.date == date).collect()
    }

    pub fn list_all_events(&self) -> Vec<&Event> {
        self.events.iter().collect()
    }

    pub fn count_events(&self) -> usize {
        self.events.len()
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    title: String,
    date: Date,
    description: String,
}

impl Event {
    pub fn new(title: String, date: Date, description: String) -> Self {
        Event { title, date, description }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Date {
    year: u32,
    month: u8,
    day: u8,
}

impl Date {
    pub fn new(year: u32, month: u8, day: u8) -> Self {
        Date { year, month, day }
    }
}
