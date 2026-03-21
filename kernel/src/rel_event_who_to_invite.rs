extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct Event {
    name: String,
    date: String,
    location: String,
    guests: Vec<String>,
}

impl Event {
    pub fn new(name: &str, date: &str, location: &str) -> Self {
        Event {
            name: String::from(name),
            date: String::from(date),
            location: String::from(location),
            guests: Vec::new(),
        }
    }

    pub fn add_guest(&mut self, guest_name: &str) {
        self.guests.push(String::from(guest_name));
    }

    pub fn remove_guest(&mut self, guest_name: &str) -> bool {
        if let Some(index) = self.guests.iter().position(|g| g == guest_name) {
            self.guests.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_guests(&self) -> Vec<String> {
        self.guests.clone()
    }

    pub fn get_event_details(&self) -> String {
        let mut details = String::new();
        details.push_str("Event Name: ");
        details.push_str(&self.name);
        details.push('\n');
        details.push_str("Date: ");
        details.push_str(&self.date);
        details.push('\n');
        details.push_str("Location: ");
        details.push_str(&self.location);
        details.push('\n');
        details.push_str("Guests:\n");
        for guest in &self.guests {
            details.push_str(guest);
            details.push('\n');
        }
        details
    }

    pub fn is_guest_attending(&self, guest_name: &str) -> bool {
        self.guests.contains(&String::from(guest_name))
    }
}
