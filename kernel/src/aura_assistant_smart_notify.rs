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

pub struct AuraAssistantSmartNotify {
    notifications: Vec<String>,
}

impl AuraAssistantSmartNotify {
    pub fn new() -> Self {
        AuraAssistantSmartNotify {
            notifications: Vec::new(),
        }
    }

    pub fn add_notification(&mut self, message: &str) {
        self.notifications.push(String::from(message));
    }

    pub fn remove_notification(&mut self, index: usize) -> Option<String> {
        if index < self.notifications.len() {
            Some(self.notifications.remove(index))
        } else {
            None
        }
    }

    pub fn get_notifications(&self) -> &Vec<String> {
        &self.notifications
    }

    pub fn clear_notifications(&mut self) {
        self.notifications.clear();
    }

    pub fn has_notification(&self, message: &str) -> bool {
        self.notifications.iter().any(|n| n == message)
    }
}
