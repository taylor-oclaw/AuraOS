extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialize the module
}

pub extern "C" fn rust_ffi_exit() {
    // Cleanup the module
}

pub struct AuraNotificationHub {
    notifications: Vec<String>,
}

impl AuraNotificationHub {
    pub fn new() -> Self {
        AuraNotificationHub {
            notifications: Vec::new(),
        }
    }

    pub fn add_notification(&mut self, notification: String) {
        self.notifications.push(notification);
    }

    pub fn remove_notification(&mut self, index: usize) -> Option<String> {
        if index < self.notifications.len() {
            Some(self.notifications.remove(index))
        } else {
            None
        }
    }

    pub fn get_notifications(&self) -> &[String] {
        &self.notifications
    }

    pub fn clear_notifications(&mut self) {
        self.notifications.clear();
    }

    pub fn notification_count(&self) -> usize {
        self.notifications.len()
    }
}
