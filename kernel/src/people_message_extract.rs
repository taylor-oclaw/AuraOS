extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PeopleMessageExtract {
    messages: Vec<String>,
}

impl PeopleMessageExtract {
    pub fn new() -> Self {
        PeopleMessageExtract {
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, message: String) {
        self.messages.push(message);
    }

    pub fn get_messages(&self) -> &Vec<String> {
        &self.messages
    }

    pub fn clear_messages(&mut self) {
        self.messages.clear();
    }

    pub fn find_message(&self, keyword: &str) -> Option<&String> {
        self.messages.iter().find(|msg| msg.contains(keyword))
    }

    pub fn count_messages(&self) -> usize {
        self.messages.len()
    }
}
