extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ConversationHistory {
    history: Vec<String>,
}

impl ConversationHistory {
    pub fn new() -> Self {
        ConversationHistory {
            history: Vec::new(),
        }
    }

    pub fn add_message(&mut self, message: String) {
        self.history.push(message);
    }

    pub fn get_last_message(&self) -> Option<&String> {
        self.history.last()
    }

    pub fn get_all_messages(&self) -> &[String] {
        &self.history
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    pub fn message_count(&self) -> usize {
        self.history.len()
    }
}
