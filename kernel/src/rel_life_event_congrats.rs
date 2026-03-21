extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
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
        println!("Congratulations {} on your {}! You are now {} years old.", self.name, self.event_name, self.age);
    }

    pub fn update_event(&mut self, new_event_name: &str) {
        self.event_name = String::from(new_event_name);
        println!("Event name updated to {}", self.event_name);
    }

    pub fn add_guest(&mut self, guest_name: &str) {
        self.guests.push(String::from(guest_name));
        println!("Guest {} added.", guest_name);
    }

    pub fn remove_guest(&mut self, guest_name: &str) {
        if let Some(index) = self.guests.iter().position(|g| g == guest_name) {
            self.guests.remove(index);
            println!("Guest {} removed.", guest_name);
        } else {
            println!("Guest {} not found.", guest_name);
        }
    }

    pub fn list_guests(&self) {
        if self.guests.is_empty() {
            println!("No guests.");
        } else {
            println!("Guests:");
            for guest in &self.guests {
                println!("{}", guest);
            }
        }
    }
}
