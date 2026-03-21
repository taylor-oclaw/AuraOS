extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn tone_frustration_defuse_init() {
    // Initialization logic for the module
}

pub extern "C" fn tone_frustration_defuse_exit() {
    // Cleanup logic for the module
}

pub struct ToneFrustrationDefuser {
    messages: Vec<String>,
    threshold: usize,
}

impl ToneFrustrationDefuser {
    pub fn new(threshold: usize) -> Self {
        ToneFrustrationDefuser {
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

    pub fn is_frustrated(&self) -> bool {
        self.messages.len() >= self.threshold
    }
}
