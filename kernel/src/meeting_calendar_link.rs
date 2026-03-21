extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeetingCalendarLink {
    events: Vec<Event>,
}

impl MeetingCalendarLink {
    pub fn new() -> Self {
        MeetingCalendarLink {
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

    pub fn update_event_title(&mut self, old_title: &str, new_title: String) -> bool {
        if let Some(event) = self.events.iter_mut().find(|e| e.title == old_title) {
            event.title = new_title;
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    title: String,
    date: Date,
    time: Time,
    location: Option<String>,
    description: Option<String>,
}

impl Event {
    pub fn new(title: String, date: Date, time: Time) -> Self {
        Event {
            title,
            date,
            time,
            location: None,
            description: None,
        }
    }

    pub fn set_location(&mut self, location: String) {
        self.location = Some(location);
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

impl Date {
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Date { year, month, day }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Time {
    hour: u8,
    minute: u8,
}
