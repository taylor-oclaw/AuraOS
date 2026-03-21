extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut locator = MDMRemoteLocate::new();
    locator.add_device("device1", "location1");
    locator.add_device("device2", "location2");

    if let Some(location) = locator.get_location("device1") {
        println!("Device1 is located at {}", location);
    }

    locator.remove_device("device2");

    if let None = locator.get_location("device2") {
        println!("Device2 has been removed.");
    }
}

pub struct MDMRemoteLocate {
    devices: Vec<(String, String)>,
}

impl MDMRemoteLocate {
    pub fn new() -> Self {
        MDMRemoteLocate {
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device_name: &str, location: &str) {
        self.devices.push((String::from(device_name), String::from(location)));
    }

    pub fn get_location(&self, device_name: &str) -> Option<&str> {
        for (name, loc) in &self.devices {
            if name == device_name {
                return Some(loc);
            }
        }
        None
    }

    pub fn remove_device(&mut self, device_name: &str) {
        self.devices.retain(|(name, _)| name != device_name);
    }

    pub fn list_devices(&self) -> Vec<&str> {
        self.devices.iter().map(|(name, _)| name.as_str()).collect()
    }

    pub fn count_devices(&self) -> usize {
        self.devices.len()
    }
}
