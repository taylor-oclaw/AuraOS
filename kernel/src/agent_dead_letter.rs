extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentDeadLetter {
    messages: Vec<String>,
}

impl AgentDeadLetter {
    pub fn new() -> Self {
        AgentDeadLetter {
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, message: String) {
        self.messages.push(message);
    }

    pub fn get_messages(&self) -> &Vec<String> {
        &self.messages
    }

    pub fn remove_message(&mut self, index: usize) -> Option<String> {
        if index < self.messages.len() {
            Some(self.messages.remove(index))
        } else {
            None
        }
    }

    pub fn clear_messages(&mut self) {
        self.messages.clear();
    }

    pub fn message_count(&self) -> usize {
        self.messages.len()
    }
}
