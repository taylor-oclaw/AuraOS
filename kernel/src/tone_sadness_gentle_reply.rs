extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ToneSadnessGentleReply {
    messages: Vec<String>,
}

impl ToneSadnessGentleReply {
    pub fn new() -> Self {
        ToneSadnessGentleReply {
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

    pub fn has_messages(&self) -> bool {
        !self.messages.is_empty()
    }

    pub fn find_message(&self, keyword: &str) -> Option<&String> {
        self.messages.iter().find(|msg| msg.contains(keyword))
    }
}
