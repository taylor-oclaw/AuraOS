extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn init_module() -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn cleanup_module() {}

struct NotificationDrivingAware {
    notifications: Vec<String>,
    driving_mode: bool,
}

impl NotificationDrivingAware {
    pub fn new() -> Self {
        NotificationDrivingAware {
            notifications: Vec::new(),
            driving_mode: false,
        }
    }

    pub fn add_notification(&mut self, notification: String) {
        if !self.driving_mode {
            self.notifications.push(notification);
        }
    }

    pub fn get_notifications(&self) -> &[String] {
        &self.notifications
    }

    pub fn clear_notifications(&mut self) {
        self.notifications.clear();
    }

    pub fn set_driving_mode(&mut self, mode: bool) {
        self.driving_mode = mode;
    }

    pub fn is_driving_mode(&self) -> bool {
        self.driving_mode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_driving_aware() {
        let mut nda = NotificationDrivingAware::new();
        assert_eq!(nda.get_notifications().len(), 0);
        assert!(!nda.is_driving_mode());

        nda.add_notification(String::from("Test notification"));
        assert_eq!(nda.get_notifications().len(), 1);

        nda.set_driving_mode(true);
        nda.add_notification(String::from("This should not be added"));
        assert_eq!(nda.get_notifications().len(), 1);

        nda.clear_notifications();
        assert_eq!(nda.get_notifications().len(), 0);
    }
}
