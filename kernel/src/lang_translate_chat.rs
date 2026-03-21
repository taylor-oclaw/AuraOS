extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct LangTranslateChat {
    messages: Vec<String>,
}

impl LangTranslateChat {
    pub fn new() -> Self {
        LangTranslateChat {
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, message: String) {
        self.messages.push(message);
    }

    pub fn get_messages(&self) -> &Vec<String> {
        &self.messages
    }

    pub fn translate_message(&self, index: usize, target_language: &str) -> Option<String> {
        if let Some(message) = self.messages.get(index) {
            // Simulate translation logic
            match target_language {
                "en" => Some(String::from("info")),
                "fr" => Some(String::from("info")),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn clear_messages(&mut self) {
        self.messages.clear();
    }

    pub fn count_messages(&self) -> usize {
        self.messages.len()
    }
}
