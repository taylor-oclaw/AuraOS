extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct SmartHomeHubCore {
    devices: Vec<String>,
    status: String,
}

impl SmartHomeHubCore {
    pub fn new() -> Self {
        SmartHomeHubCore {
            devices: Vec::new(),
            status: String::from("Idle"),
        }
    }

    pub fn add_device(&mut self, device_name: &str) {
        self.devices.push(String::from(device_name));
    }

    pub fn remove_device(&mut self, device_name: &str) -> bool {
        if let Some(index) = self.devices.iter().position(|d| d == device_name) {
            self.devices.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_devices(&self) -> Vec<String> {
        self.devices.clone()
    }

    pub fn set_status(&mut self, status: &str) {
        self.status = String::from(status);
    }

    pub fn get_status(&self) -> String {
        self.status.clone()
    }
}
