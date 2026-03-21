extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct CrossDeviceNotificationSync {
    device_ids: Vec<u32>,
    notifications: Vec<String>,
}

impl CrossDeviceNotificationSync {
    pub fn new(device_ids: Vec<u32>) -> Self {
        CrossDeviceNotificationSync {
            device_ids,
            notifications: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device_id: u32) {
        if !self.device_ids.contains(&device_id) {
            self.device_ids.push(device_id);
        }
    }

    pub fn remove_device(&mut self, device_id: u32) {
        self.device_ids.retain(|&id| id != device_id);
    }

    pub fn send_notification(&mut self, message: String) {
        self.notifications.push(message);
    }

    pub fn get_notifications(&self) -> Vec<String> {
        self.notifications.clone()
    }

    pub fn clear_notifications(&mut self) {
        self.notifications.clear();
    }
}
