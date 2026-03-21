extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CalendarAttendeeSuggest {
    attendees: Vec<String>,
}

impl CalendarAttendeeSuggest {
    pub fn new() -> Self {
        CalendarAttendeeSuggest {
            attendees: Vec::new(),
        }
    }

    pub fn add_attendee(&mut self, name: &str) {
        self.attendees.push(String::from(name));
    }

    pub fn remove_attendee(&mut self, name: &str) -> bool {
        if let Some(index) = self.attendees.iter().position(|x| x == name) {
            self.attendees.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_attendees(&self) -> Vec<String> {
        self.attendees.clone()
    }

    pub fn find_attendee(&self, name: &str) -> bool {
        self.attendees.contains(&String::from(name))
    }

    pub fn suggest_attendees(&self, prefix: &str) -> Vec<String> {
        self.attendees
            .iter()
            .filter(|x| x.starts_with(prefix))
            .cloned()
            .collect()
    }
}
