extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct ToneGreetingMatch {
    greetings: Vec<String>,
}

impl ToneGreetingMatch {
    pub fn new() -> Self {
        ToneGreetingMatch {
            greetings: vec![
                String::from("Hello!"),
                String::from("Hi there!"),
                String::from("Greetings!"),
                String::from("Welcome!"),
                String::from("Good day!"),
            ],
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

    pub fn get_greeting(&self, index: usize) -> Option<&String> {
        self.greetings.get(index)
    }

    pub fn list_greetings(&self) -> &[String] {
        &self.greetings
    }

    pub fn find_greeting(&self, keyword: &str) -> Vec<&String> {
        self.greetings.iter().filter(|g| g.contains(keyword)).collect()
    }
}
