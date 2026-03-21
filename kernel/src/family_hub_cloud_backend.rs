extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut family_hub = FamilyHubCloudBackend::new();

    // Example usage of methods
    family_hub.add_device(String::from("Smart Thermostat"));
    family_hub.add_device(String::from("Security Camera"));
    family_hub.remove_device(String::from("Smart Thermostat"));
    family_hub.list_devices();
    let status = family_hub.get_device_status(String::from("Security Camera"));

    loop {}
}

pub struct FamilyHubCloudBackend {
    devices: Vec<String>,
}

impl FamilyHubCloudBackend {
    pub fn new() -> Self {
        FamilyHubCloudBackend {
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device_name: String) {
        if !self.devices.contains(&device_name) {
            self.devices.push(device_name);
        }
    }

    pub fn remove_device(&mut self, device_name: String) {
        self.devices.retain(|d| d != &device_name);
    }

    pub fn list_devices(&self) -> Vec<String> {
        self.devices.clone()
    }

    pub fn get_device_status(&self, device_name: String) -> Option<&String> {
        self.devices.iter().find(|&&d| d == device_name)
    }
}
