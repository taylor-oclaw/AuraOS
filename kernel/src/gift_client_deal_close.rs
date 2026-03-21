extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn gift_client_thank_you_init() {
    // Initialization logic for the module
}

pub extern "C" fn gift_client_thank_you_exit() {
    // Cleanup logic for the module
}

pub struct GiftClientThankYou {
    messages: Vec<String>,
    count: usize,
}

impl GiftClientThankYou {
    pub fn new() -> Self {
        GiftClientThankYou {
            messages: Vec::new(),
            count: 0,
        }
    }

    pub fn add_message(&mut self, message: String) {
        self.messages.push(message);
        self.count += 1;
    }

    pub fn get_messages(&self) -> &Vec<String> {
        &self.messages
    }

    pub fn get_count(&self) -> usize {
        self.count
    }

    pub fn clear_messages(&mut self) {
        self.messages.clear();
        self.count = 0;
    }

    pub fn find_message(&self, keyword: &str) -> Option<&String> {
        self.messages.iter().find(|msg| msg.contains(keyword))
    }
}
