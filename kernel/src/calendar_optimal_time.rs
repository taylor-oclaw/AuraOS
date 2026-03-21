extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CalendarOptimalTime {
    events: Vec<Event>,
}

impl CalendarOptimalTime {
    pub fn new() -> Self {
        CalendarOptimalTime {
            events: Vec::new(),
        }
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
        self.events.sort_by_key(|e| e.start_time);
    }

    pub fn remove_event(&mut self, title: &str) -> bool {
        let index = self.events.iter().position(|e| e.title == title);
        if let Some(i) = index {
            self.events.remove(i);
            true
        } else {
            false
        }
    }

    pub fn get_events_by_date(&self, date: u32) -> Vec<&Event> {
        self.events.iter().filter(|e| e.date == date).collect()
    }

    pub fn find_next_event(&self, current_time: u32) -> Option<&Event> {
        self.events
            .iter()
            .find(|e| e.start_time > current_time)
    }

    pub fn get_total_events(&self) -> usize {
        self.events.len()
    }
}

pub struct Event {
    title: String,
    date: u32,
    start_time: u32,
    end_time: u32,
}

impl Event {
    pub fn new(title: String, date: u32, start_time: u32, end_time: u32) -> Self {
        Event {
            title,
            date,
            start_time,
            end_time,
        }
    }
}
