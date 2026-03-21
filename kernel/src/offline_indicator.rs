extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct OfflineIndicator {
    status: bool,
    messages: Vec<String>,
}

impl OfflineIndicator {
    pub fn new() -> Self {
        OfflineIndicator {
            status: true,
            messages: Vec::new(),
        }
    }

    pub fn set_online(&mut self) {
        self.status = true;
    }

    pub fn set_offline(&mut self) {
        self.status = false;
    }

    pub fn is_online(&self) -> bool {
        self.status
    }

    pub fn add_message(&mut self, message: String) {
        self.messages.push(message);
    }

    pub fn get_messages(&self) -> &Vec<String> {
        &self.messages
    }
}
