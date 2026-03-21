extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rel_holiday_greet_init() {
    // Initialization logic for the module
}

pub extern "C" fn rel_holiday_greet_exit() {
    // Cleanup logic for the module
}

pub struct HolidayGreeting {
    greetings: Vec<String>,
}

impl HolidayGreeting {
    pub fn new() -> Self {
        HolidayGreeting {
            greetings: Vec::new(),
        }
    }

    pub fn add_greeting(&mut self, greeting: String) {
        self.greetings.push(greeting);
    }

    pub fn get_greeting(&self, index: usize) -> Option<&String> {
        self.greetings.get(index)
    }

    pub fn remove_greeting(&mut self, index: usize) -> Option<String> {
        if index < self.greetings.len() {
            Some(self.greetings.remove(index))
        } else {
            None
        }
    }

    pub fn list_greetings(&self) -> &[String] {
        &self.greetings
    }

    pub fn count_greetings(&self) -> usize {
        self.greetings.len()
    }
}
