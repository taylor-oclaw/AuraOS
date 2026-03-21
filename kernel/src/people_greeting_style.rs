extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn init_module() -> i32 {
    0
}

pub extern "C" fn cleanup_module() -> i32 {
    0
}

pub struct PeopleGreetingStyle {
    greetings: Vec<String>,
}

impl PeopleGreetingStyle {
    pub fn new() -> Self {
        PeopleGreetingStyle {
            greetings: Vec::new(),
        }
    }

    pub fn add_greeting(&mut self, greeting: &str) {
        self.greetings.push(String::from(greeting));
    }

    pub fn remove_greeting(&mut self, index: usize) -> Option<String> {
        if index < self.greetings.len() {
            Some(self.greetings.remove(index))
        } else {
            None
        }
    }

    pub fn get_greeting(&self, index: usize) -> Option<&str> {
        self.greetings.get(index).map(|s| s.as_str())
    }

    pub fn list_greetings(&self) -> Vec<&str> {
        self.greetings.iter().map(|s| s.as_str()).collect()
    }

    pub fn count_greetings(&self) -> usize {
        self.greetings.len()
    }
}
