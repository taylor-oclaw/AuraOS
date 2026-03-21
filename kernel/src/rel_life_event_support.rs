extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct LifeEventSupport {
    events: Vec<String>,
}

impl LifeEventSupport {
    pub fn new() -> Self {
        LifeEventSupport {
            events: Vec::new(),
        }
    }

    pub fn add_event(&mut self, event: &str) {
        self.events.push(String::from(event));
    }

    pub fn remove_event(&mut self, index: usize) -> Option<String> {
        if index < self.events.len() {
            Some(self.events.remove(index))
        } else {
            None
        }
    }

    pub fn get_event(&self, index: usize) -> Option<&String> {
        self.events.get(index)
    }

    pub fn list_events(&self) -> &Vec<String> {
        &self.events
    }

    pub fn clear_events(&mut self) {
        self.events.clear();
    }
}
