extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct FamilyNotification {
    notifications: Vec<String>,
}

impl FamilyNotification {
    pub fn new() -> Self {
        FamilyNotification {
            notifications: Vec::new(),
        }
    }

    pub fn add_notification(&mut self, message: &str) {
        self.notifications.push(String::from(message));
    }

    pub fn get_notifications(&self) -> &[String] {
        &self.notifications
    }

    pub fn clear_notifications(&mut self) {
        self.notifications.clear();
    }

    pub fn has_notifications(&self) -> bool {
        !self.notifications.is_empty()
    }

    pub fn remove_notification(&mut self, index: usize) -> Option<String> {
        if index < self.notifications.len() {
            Some(self.notifications.remove(index))
        } else {
            None
        }
    }
}
