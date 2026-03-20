extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn a2a_push_notify_init() {
    // Initialization logic for the module
}

pub extern "C" fn a2a_push_notify_exit() {
    // Cleanup logic for the module
}

pub struct A2APushNotify {
    notifications: Vec<String>,
}

impl A2APushNotify {
    pub fn new() -> Self {
        A2APushNotify {
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

    pub fn get_notifications(&self) -> &[String] {
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
    fn test_a2a_push_notify() {
        let mut notify = A2APushNotify::new();

        assert!(!notify.has_notifications());
        assert_eq!(notify.get_notifications().len(), 0);

        notify.add_notification("Hello, World!");
        assert!(notify.has_notifications());
        assert_eq!(notify.get_notifications().len(), 1);
        assert_eq!(notify.get_notifications()[0], "Hello, World!");

        let removed = notify.remove_notification(0);
        assert_eq!(removed, Some(String::from("Hello, World!")));
        assert!(!notify.has_notifications());
        assert_eq!(notify.get_notifications().len(), 0);

        notify.add_notification("Test Notification");
        notify.clear_notifications();
        assert!(!notify.has_notifications());
        assert_eq!(notify.get_notifications().len(), 0);
    }
}
