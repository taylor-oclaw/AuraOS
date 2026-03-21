extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let devfs = CompatLinuxDevFS::new();
    devfs.create_device("device1");
    devfs.create_device("device2");
    devfs.list_devices();
}

pub struct CompatLinuxDevFS {
    devices: Vec<String>,
}

impl CompatLinuxDevFS {
    pub fn new() -> Self {
        CompatLinuxDevFS {
            devices: Vec::new(),
        }
    }

    pub fn create_device(&mut self, name: &str) {
        self.devices.push(String::from(name));
    }

    pub fn remove_device(&mut self, name: &str) {
        if let Some(index) = self.devices.iter().position(|d| d == name) {
            self.devices.remove(index);
        }
    }

    pub fn list_devices(&self) -> Vec<String> {
        self.devices.clone()
    }

    pub fn device_exists(&self, name: &str) -> bool {
        self.devices.contains(&String::from(name))
    }

    pub fn get_device_count(&self) -> usize {
        self.devices.len()
    }
}
