extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CalendarTimezoneSmart {
    timezone: String,
    events: Vec<Event>,
}

impl CalendarTimezoneSmart {
    pub fn new(timezone: &str) -> Self {
        CalendarTimezoneSmart {
            timezone: String::from(timezone),
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

    pub fn get_events_in_timezone(&self) -> Vec<&Event> {
        self.events.iter().collect()
    }

    pub fn find_event_by_name(&self, name: &str) -> Option<&Event> {
        self.events.iter().find(|event| event.name == name)
    }

    pub fn update_timezone(&mut self, new_timezone: &str) {
        self.timezone = String::from(new_timezone);
    }
}

pub struct Event {
    name: String,
    date: String,
    time: String,
}

impl Event {
    pub fn new(name: &str, date: &str, time: &str) -> Self {
        Event {
            name: String::from(name),
            date: String::from(date),
            time: String::from(time),
        }
    }
}
