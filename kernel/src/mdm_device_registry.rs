extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MdmDeviceRegistry {
    devices: Vec<String>,
}

impl MdmDeviceRegistry {
    pub fn new() -> Self {
        MdmDeviceRegistry {
            devices: Vec::new(),
        }
    }

    pub fn register_device(&mut self, device_name: &str) {
        if !self.devices.contains(&String::from(device_name)) {
            self.devices.push(String::from(device_name));
        }
    }

    pub fn unregister_device(&mut self, device_name: &str) {
        self.devices.retain(|dev| dev != device_name);
    }

    pub fn list_devices(&self) -> Vec<String> {
        self.devices.clone()
    }

    pub fn is_device_registered(&self, device_name: &str) -> bool {
        self.devices.contains(&String::from(device_name))
    }

    pub fn count_devices(&self) -> usize {
        self.devices.len()
    }
}
