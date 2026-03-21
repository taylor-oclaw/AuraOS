extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct SmartHomeVoiceControl {
    devices: Vec<String>,
    commands: Vec<String>,
}

impl SmartHomeVoiceControl {
    pub fn new() -> Self {
        SmartHomeVoiceControl {
            devices: Vec::new(),
            commands: Vec::new(),
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

    pub fn add_command(&mut self, command: &str) {
        self.commands.push(String::from(command));
    }

    pub fn execute_command(&self, command: &str) -> bool {
        self.commands.contains(&String::from(command))
    }
}
