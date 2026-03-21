extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct SmartHomeRoutineNight {
    devices: Vec<String>,
    current_device_index: usize,
}

impl SmartHomeRoutineNight {
    pub fn new(devices: Vec<String>) -> Self {
        SmartHomeRoutineNight {
            devices,
            current_device_index: 0,
        }
    }

    pub fn add_device(&mut self, device_name: String) {
        self.devices.push(device_name);
    }

    pub fn remove_device(&mut self, device_name: &str) -> bool {
        if let Some(index) = self.devices.iter().position(|d| d == device_name) {
            self.devices.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_current_device(&self) -> Option<&String> {
        self.devices.get(self.current_device_index)
    }

    pub fn next_device(&mut self) {
        if !self.devices.is_empty() {
            self.current_device_index = (self.current_device_index + 1) % self.devices.len();
        }
    }

    pub fn previous_device(&mut self) {
        if !self.devices.is_empty() {
            if self.current_device_index == 0 {
                self.current_device_index = self.devices.len() - 1;
            } else {
                self.current_device_index -= 1;
            }
        }
    }
}
