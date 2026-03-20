extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_nvme_driver_init() {
}

pub extern "C" fn rust_nvme_driver_exit() {
}

pub struct NvmeDriver {
    devices: Vec<String>,
    current_device_index: usize,
}

impl NvmeDriver {
    pub fn new() -> Self {
        NvmeDriver {
            devices: Vec::new(),
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

    pub fn list_devices(&self) -> Vec<String> {
        self.devices.clone()
    }

    pub fn select_next_device(&mut self) {
        if !self.devices.is_empty() {
            self.current_device_index = (self.current_device_index + 1) % self.devices.len();
        }
    }

    pub fn get_current_device(&self) -> Option<&String> {
        if self.devices.is_empty() {
            None
        } else {
            Some(&self.devices[self.current_device_index])
        }
    }
}
