extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn init_module() -> i32 {
    0
}

pub extern "C" fn cleanup_module() -> i32 {
    0
}

pub struct LifeEventCondolence {
    messages: Vec<String>,
}

impl LifeEventCondolence {
    pub fn new() -> Self {
        LifeEventCondolence {
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

    pub fn count_messages(&self) -> usize {
        self.messages.len()
    }
}
