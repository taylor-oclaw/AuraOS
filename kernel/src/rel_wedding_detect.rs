extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rel_wedding_detect_init() -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn rel_wedding_detect_exit() -> i32 {
    0
}

pub struct WeddingDetector {
    people: Vec<String>,
    events: Vec<String>,
}

impl WeddingDetector {
    pub fn new() -> Self {
        WeddingDetector {
            people: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn add_person(&mut self, name: String) {
        self.people.push(name);
    }

    pub fn remove_person(&mut self, name: &str) {
        if let Some(index) = self.people.iter().position(|x| x == name) {
            self.people.remove(index);
        }
    }

    pub fn add_event(&mut self, event_name: String) {
        self.events.push(event_name);
    }

    pub fn remove_event(&mut self, event_name: &str) {
        if let Some(index) = self.events.iter().position(|x| x == event_name) {
            self.events.remove(index);
        }
    }

    pub fn list_people(&self) -> Vec<String> {
        self.people.clone()
    }

    pub fn list_events(&self) -> Vec<String> {
        self.events.clone()
    }
}
