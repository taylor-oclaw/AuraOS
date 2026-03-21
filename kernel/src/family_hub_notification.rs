extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct FamilyHubNotification {
    notifications: Vec<String>,
}

impl FamilyHubNotification {
    pub fn new() -> Self {
        FamilyHubNotification {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_remove_notification() {
        let mut hub = FamilyHubNotification::new();
        hub.add_notification("Test Notification");
        assert_eq!(hub.get_notifications().len(), 1);
        assert_eq!(hub.remove_notification(0), Some(String::from("Test Notification")));
        assert_eq!(hub.get_notifications().len(), 0);
    }

    #[test]
    fn test_clear_notifications() {
        let mut hub = FamilyHubNotification::new();
        hub.add_notification("Test Notification 1");
        hub.add_notification("Test Notification 2");
        hub.clear_notifications();
        assert_eq!(hub.get_notifications().len(), 0);
    }

    #[test]
    fn test_has_notification() {
        let mut hub = FamilyHubNotification::new();
        hub.add_notification("Test Notification");
        assert!(hub.has_notification("Test Notification"));
        assert!(!hub.has_notification("Non-existent Notification"));
    }
}
