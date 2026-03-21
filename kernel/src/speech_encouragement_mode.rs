extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SpeechEncouragementMode {
    messages: Vec<String>,
    current_index: usize,
}

impl SpeechEncouragementMode {
    pub fn new() -> Self {
        let mut messages = Vec::new();
        messages.push(String::from("You are doing great!"));
        messages.push(String::from("Keep pushing forward!"));
        messages.push(String::from("Believe in yourself!"));
        messages.push(String::from("Success is within reach!"));
        messages.push(String::from("Never give up!"));

        SpeechEncouragementMode {
            messages,
            current_index: 0,
        }
    }

    pub fn get_current_message(&self) -> &str {
        &self.messages[self.current_index]
    }

    pub fn next_message(&mut self) {
        self.current_index = (self.current_index + 1) % self.messages.len();
    }

    pub fn previous_message(&mut self) {
        if self.current_index == 0 {
            self.current_index = self.messages.len() - 1;
        } else {
            self.current_index -= 1;
        }
    }

    pub fn add_message(&mut self, message: String) {
        self.messages.push(message);
    }

    pub fn remove_message(&mut self, index: usize) -> Option<String> {
        if index < self.messages.len() {
            Some(self.messages.remove(index))
        } else {
            None
        }
    }
}
