extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialize the module if needed
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Clean up the module if needed
}

pub struct AuraAssistantFollowUp {
    history: Vec<String>,
    max_history_size: usize,
}

impl AuraAssistantFollowUp {
    pub fn new(max_history_size: usize) -> Self {
        AuraAssistantFollowUp {
            history: Vec::new(),
            max_history_size,
        }
    }

    pub fn add_message(&mut self, message: String) {
        if self.history.len() >= self.max_history_size {
            self.history.remove(0);
        }
        self.history.push(message);
    }

    pub fn get_last_message(&self) -> Option<&String> {
        self.history.last()
    }

    pub fn get_all_messages(&self) -> Vec<&String> {
        self.history.iter().collect()
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    pub fn history_size(&self) -> usize {
        self.history.len()
    }
}
