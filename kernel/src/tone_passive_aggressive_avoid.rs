extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn init_module() -> i32 {
    0
}

pub extern "C" fn cleanup_module() {}

struct TonePassiveAggressiveAvoid {
    messages: Vec<String>,
    threshold: usize,
}

impl TonePassiveAggressiveAvoid {
    pub fn new(threshold: usize) -> Self {
        TonePassiveAggressiveAvoid {
            messages: Vec::new(),
            threshold,
        }
    }

    pub fn add_message(&mut self, message: String) {
        if self.messages.len() < self.threshold {
            self.messages.push(message);
        }
    }

    pub fn get_messages(&self) -> &Vec<String> {
        &self.messages
    }

    pub fn clear_messages(&mut self) {
        self.messages.clear();
    }

    pub fn is_full(&self) -> bool {
        self.messages.len() >= self.threshold
    }

    pub fn remove_message(&mut self, index: usize) -> Option<String> {
        if index < self.messages.len() {
            Some(self.messages.remove(index))
        } else {
            None
        }
    }
}
