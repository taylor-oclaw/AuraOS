#![no_std]
#![feature(allocator_api)]
#![feature(const_mut_refs)]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

struct AuraNotificationHub {
    notifications: Vec<String>,
}

impl AuraNotificationHub {
    pub fn new() -> Self {
        AuraNotificationHub { notifications: Vec::new() }
    }

    pub fn add_notification(&mut self, message: String) {
        self.notifications.push(message);
    }

    pub fn get_notifications(&self) -> &Vec<String> {
        &self.notifications
    }

    pub fn clear_notifications(&mut self) {
        self.notifications.clear();
    }

    pub fn has_notifications(&self) -> bool {
        !self.notifications.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_notification() {
        let mut hub = AuraNotificationHub::new();
        hub.add_notification(String::from("Hello, world!"));
        assert_eq!(hub.get_notifications(), &vec![String::from("Hello, world!")]);
    }

    #[test]
    fn test_clear_notifications() {
        let mut hub = AuraNotificationHub::new();
        hub.add_notification(String::from("Hello, world!"));
        hub.clear_notifications();
        assert!(hub.get_notifications().is_empty());
    }

    #[test]
    fn test_has_notifications() {
        let mut hub = AuraNotificationHub::new();
        assert!(!hub.has_notifications());
        hub.add_notification(String::from("Hello, world!"));
        assert!(hub.has_notifications());
    }
}