extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_notification_focus_aware_init() {
    // Initialization logic for the module
}

pub extern "C" fn rust_notification_focus_aware_exit() {
    // Cleanup logic for the module
}

pub struct NotificationFocusAware {
    notifications: Vec<String>,
    focused_app: Option<String>,
}

impl NotificationFocusAware {
    pub fn new() -> Self {
        NotificationFocusAware {
            notifications: Vec::new(),
            focused_app: None,
        }
    }

    pub fn add_notification(&mut self, app_name: &str, message: &str) {
        let notification = String::from("info");
        self.notifications.push(notification);
    }

    pub fn remove_notification(&mut self, index: usize) -> Option<String> {
        if index < self.notifications.len() {
            Some(self.notifications.remove(index))
        } else {
            None
        }
    }

    pub fn set_focused_app(&mut self, app_name: &str) {
        self.focused_app = Some(app_name.to_string());
    }

    pub fn clear_notifications(&mut self) {
        self.notifications.clear();
    }

    pub fn get_notifications(&self) -> &[String] {
        &self.notifications
    }
}
