extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct ProfileDeviceSharedFamily {
    name: String,
    devices: Vec<String>,
    max_devices: usize,
}

impl ProfileDeviceSharedFamily {
    pub fn new(name: &str, max_devices: usize) -> Self {
        ProfileDeviceSharedFamily {
            name: String::from(name),
            devices: Vec::new(),
            max_devices,
        }
    }

    pub fn add_device(&mut self, device_name: &str) -> bool {
        if self.devices.len() < self.max_devices {
            self.devices.push(String::from(device_name));
            true
        } else {
            false
        }
    }

    pub fn remove_device(&mut self, device_name: &str) -> bool {
        let pos = self.devices.iter().position(|d| d == device_name);
        if let Some(index) = pos {
            self.devices.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_devices(&self) -> Vec<String> {
        self.devices.clone()
    }

    pub fn get_device_count(&self) -> usize {
        self.devices.len()
    }

    pub fn is_full(&self) -> bool {
        self.devices.len() >= self.max_devices
    }
}
