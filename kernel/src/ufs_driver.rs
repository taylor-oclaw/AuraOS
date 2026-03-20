extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn ufs_driver_init() -> i32 {
    // Initialization logic for the UFS driver
    0
}

#[no_mangle]
pub extern "C" fn ufs_driver_exit() {
    // Cleanup logic for the UFS driver
}

pub struct UFSDriver {
    devices: Vec<String>,
    current_device: Option<usize>,
}

impl UFSDriver {
    pub fn new() -> Self {
        UFSDriver {
            devices: Vec::new(),
            current_device: None,
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

    pub fn select_device(&mut self, device_name: &str) -> bool {
        if let Some(index) = self.devices.iter().position(|d| d == device_name) {
            self.current_device = Some(index);
            true
        } else {
            false
        }
    }

    pub fn get_current_device(&self) -> Option<&String> {
        self.current_device.map(|index| &self.devices[index])
    }
}
