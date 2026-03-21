extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    let event = LifeEventCongrats::new("John Doe", 30, "Graduation");
    event.congratulate();
    event.update_event("New Job");
    event.add_guest("Jane Smith");
    event.add_guest("Alice Johnson");
    event.remove_guest("Jane Smith");
    event.list_guests();
}

pub struct LifeEventCongrats {
    name: String,
    age: u32,
    event_name: String,
    guests: Vec<String>,
}

impl LifeEventCongrats {
    pub fn new(name: &str, age: u32, event_name: &str) -> Self {
        LifeEventCongrats {
            name: String::from(name),
            age,
            event_name: String::from(event_name),
            guests: Vec::new(),
        }
    }

    pub fn congratulate(&self) {
    }

    pub fn update_event(&mut self, new_event_name: &str) {
        self.event_name = String::from(new_event_name);
    }

    pub fn add_guest(&mut self, guest_name: &str) {
        self.guests.push(String::from(guest_name));
    }

    pub fn remove_guest(&mut self, guest_name: &str) {
        if let Some(index) = self.guests.iter().position(|g| g == guest_name) {
            self.guests.remove(index);
        } else {
        }
    }

    pub fn list_guests(&self) {
        if self.guests.is_empty() {
        } else {
            for guest in &self.guests {
            }
        }
    }
}
