extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CrossDeviceAutoConnect {
    devices: Vec<String>,
    connected_devices: Vec<String>,
}

impl CrossDeviceAutoConnect {
    pub fn new() -> Self {
        CrossDeviceAutoConnect {
            devices: Vec::new(),
            connected_devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device_name: &str) {
        if !self.devices.contains(&device_name.to_string()) {
            self.devices.push(device_name.to_string());
        }
    }

    pub fn remove_device(&mut self, device_name: &str) {
        if let Some(index) = self.devices.iter().position(|d| d == device_name) {
            self.devices.remove(index);
        }
    }

    pub fn connect_device(&mut self, device_name: &str) -> bool {
        if self.devices.contains(&device_name.to_string()) && !self.connected_devices.contains(&device_name.to_string()) {
            self.connected_devices.push(device_name.to_string());
            true
        } else {
            false
        }
    }

    pub fn disconnect_device(&mut self, device_name: &str) -> bool {
        if let Some(index) = self.connected_devices.iter().position(|d| d == device_name) {
            self.connected_devices.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_connected_devices(&self) -> Vec<String> {
        self.connected_devices.clone()
    }
}
