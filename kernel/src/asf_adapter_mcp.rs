extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AsfAdapterMcp {
    devices: Vec<String>,
    initialized: bool,
}

impl AsfAdapterMcp {
    pub fn new() -> Self {
        AsfAdapterMcp {
            devices: Vec::new(),
            initialized: false,
        }
    }

    pub fn initialize(&mut self) {
        if !self.initialized {
            // Simulate device initialization
            self.devices.push(String::from("Device1"));
            self.devices.push(String::from("Device2"));
            self.devices.push(String::from("Device3"));
            self.initialized = true;
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn list_devices(&self) -> Vec<String> {
        self.devices.clone()
    }

    pub fn add_device(&mut self, device_name: String) {
        if !self.devices.contains(&device_name) {
            self.devices.push(device_name);
        }
    }

    pub fn remove_device(&mut self, device_name: &str) {
        self.devices.retain(|d| d != device_name);
    }
}
