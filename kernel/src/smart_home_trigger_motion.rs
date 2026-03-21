extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut trigger = SmartHomeTriggerMotion::new();
    trigger.add_device("Living Room");
    trigger.add_device("Bedroom");
    trigger.enable_device("Living Room");
    trigger.disable_device("Bedroom");
    trigger.list_devices();
}

pub struct SmartHomeTriggerMotion {
    devices: Vec<String>,
    enabled_devices: Vec<String>,
}

impl SmartHomeTriggerMotion {
    pub fn new() -> Self {
        SmartHomeTriggerMotion {
            devices: Vec::new(),
            enabled_devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device_name: &str) {
        if !self.devices.contains(&String::from(device_name)) {
            self.devices.push(String::from(device_name));
        } else {
        }
    }

    pub fn remove_device(&mut self, device_name: &str) {
        if let Some(index) = self.devices.iter().position(|d| d == device_name) {
            self.devices.remove(index);
            if let Some(enabled_index) = self.enabled_devices.iter().position(|d| d == device_name) {
                self.enabled_devices.remove(enabled_index);
            }
        } else {
        }
    }

    pub fn enable_device(&mut self, device_name: &str) {
        if self.devices.contains(&String::from(device_name)) && !self.enabled_devices.contains(&String::from(device_name)) {
            self.enabled_devices.push(String::from(device_name));
        } else {
        }
    }

    pub fn disable_device(&mut self, device_name: &str) {
        if let Some(index) = self.enabled_devices.iter().position(|d| d == device_name) {
            self.enabled_devices.remove(index);
        } else {
        }
    }

    pub fn list_devices(&self) {
        for device in &self.devices {
            let status = if self.enabled_devices.contains(device) { "Enabled" } else { "Disabled" };
        }
    }
}
