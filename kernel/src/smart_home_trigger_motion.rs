extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
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
            println!("Device added: {}", device_name);
        } else {
            println!("Device already exists: {}", device_name);
        }
    }

    pub fn remove_device(&mut self, device_name: &str) {
        if let Some(index) = self.devices.iter().position(|d| d == device_name) {
            self.devices.remove(index);
            if let Some(enabled_index) = self.enabled_devices.iter().position(|d| d == device_name) {
                self.enabled_devices.remove(enabled_index);
            }
            println!("Device removed: {}", device_name);
        } else {
            println!("Device not found: {}", device_name);
        }
    }

    pub fn enable_device(&mut self, device_name: &str) {
        if self.devices.contains(&String::from(device_name)) && !self.enabled_devices.contains(&String::from(device_name)) {
            self.enabled_devices.push(String::from(device_name));
            println!("Device enabled: {}", device_name);
        } else {
            println!("Device not found or already enabled: {}", device_name);
        }
    }

    pub fn disable_device(&mut self, device_name: &str) {
        if let Some(index) = self.enabled_devices.iter().position(|d| d == device_name) {
            self.enabled_devices.remove(index);
            println!("Device disabled: {}", device_name);
        } else {
            println!("Device not found or already disabled: {}", device_name);
        }
    }

    pub fn list_devices(&self) {
        println!("Devices:");
        for device in &self.devices {
            let status = if self.enabled_devices.contains(device) { "Enabled" } else { "Disabled" };
            println!("{} - {}", device, status);
        }
    }
}
