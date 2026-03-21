extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct CrossDeviceScreenMirror {
    device_list: Vec<String>,
    current_device_index: usize,
}

impl CrossDeviceScreenMirror {
    pub fn new() -> Self {
        CrossDeviceScreenMirror {
            device_list: Vec::new(),
            current_device_index: 0,
        }
    }

    pub fn add_device(&mut self, device_name: &str) {
        self.device_list.push(String::from(device_name));
    }

    pub fn remove_device(&mut self, device_name: &str) -> bool {
        if let Some(index) = self.device_list.iter().position(|d| d == device_name) {
            self.device_list.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_current_device(&self) -> Option<&String> {
        self.device_list.get(self.current_device_index)
    }

    pub fn switch_to_next_device(&mut self) {
        if !self.device_list.is_empty() {
            self.current_device_index = (self.current_device_index + 1) % self.device_list.len();
        }
    }

    pub fn list_devices(&self) -> &Vec<String> {
        &self.device_list
    }
}
