extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SmartHomeDeviceDiscover {
    devices: Vec<String>,
}

impl SmartHomeDeviceDiscover {
    pub fn new() -> Self {
        SmartHomeDeviceDiscover {
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device_name: &str) {
        self.devices.push(String::from(device_name));
    }

    pub fn remove_device(&mut self, device_name: &str) {
        if let Some(index) = self.devices.iter().position(|d| d == device_name) {
            self.devices.remove(index);
        }
    }

    pub fn list_devices(&self) -> Vec<String> {
        self.devices.clone()
    }

    pub fn find_device(&self, device_name: &str) -> Option<&String> {
        self.devices.iter().find(|&&d| d == device_name)
    }

    pub fn count_devices(&self) -> usize {
        self.devices.len()
    }
}
