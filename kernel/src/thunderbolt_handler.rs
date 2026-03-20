extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn thunderbolt_handler_init() {
    // Initialization logic for the thunderbolt handler module
}

#[no_mangle]
pub extern "C" fn thunderbolt_handler_exit() {
    // Cleanup logic for the thunderbolt handler module
}

pub struct ThunderboltHandler {
    devices: Vec<String>,
    active_device: Option<usize>,
}

impl ThunderboltHandler {
    pub fn new() -> Self {
        ThunderboltHandler {
            devices: Vec::new(),
            active_device: None,
        }
    }

    pub fn add_device(&mut self, device_name: &str) {
        self.devices.push(String::from(device_name));
    }

    pub fn remove_device(&mut self, device_name: &str) -> bool {
        if let Some(index) = self.devices.iter().position(|d| d == device_name) {
            self.devices.remove(index);
            if self.active_device == Some(index) {
                self.active_device = None;
            }
            true
        } else {
            false
        }
    }

    pub fn set_active_device(&mut self, device_name: &str) -> bool {
        if let Some(index) = self.devices.iter().position(|d| d == device_name) {
            self.active_device = Some(index);
            true
        } else {
            false
        }
    }

    pub fn get_active_device(&self) -> Option<&String> {
        self.active_device.map(|index| &self.devices[index])
    }

    pub fn list_devices(&self) -> &[String] {
        &self.devices
    }
}
