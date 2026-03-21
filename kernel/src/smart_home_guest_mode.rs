extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct SmartHomeGuestMode {
    devices: Vec<String>,
    is_active: bool,
}

impl SmartHomeGuestMode {
    pub fn new() -> Self {
        SmartHomeGuestMode {
            devices: Vec::new(),
            is_active: false,
        }
    }

    pub fn activate(&mut self) {
        if !self.is_active {
            self.is_active = true;
            // Logic to activate guest mode
        }
    }

    pub fn deactivate(&mut self) {
        if self.is_active {
            self.is_active = false;
            // Logic to deactivate guest mode
        }
    }

    pub fn add_device(&mut self, device: String) {
        if !self.devices.contains(&device) {
            self.devices.push(device);
            // Logic to add a new device
        }
    }

    pub fn remove_device(&mut self, device: &str) {
        if let Some(index) = self.devices.iter().position(|d| d == device) {
            self.devices.remove(index);
            // Logic to remove a device
        }
    }

    pub fn list_devices(&self) -> Vec<String> {
        self.devices.clone()
    }
}
