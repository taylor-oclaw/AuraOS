extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[allow(non_camel_case_types)]
pub struct cross_device_proximity_detect {
    devices: Vec<String>,
    proximity_threshold: u32,
}

impl cross_device_proximity_detect {
    pub fn new(devices: Vec<String>, proximity_threshold: u32) -> Self {
        cross_device_proximity_detect {
            devices,
            proximity_threshold,
        }
    }

    pub fn add_device(&mut self, device_name: String) {
        if !self.devices.contains(&device_name) {
            self.devices.push(device_name);
        }
    }

    pub fn remove_device(&mut self, device_name: &str) -> bool {
        let index = self.devices.iter().position(|x| x == device_name);
        match index {
            Some(i) => {
                self.devices.remove(i);
                true
            },
            None => false,
        }
    }

    pub fn get_device_count(&self) -> usize {
        self.devices.len()
    }

    pub fn is_within_proximity(&self, distance: u32) -> bool {
        distance <= self.proximity_threshold
    }

    pub fn update_proximity_threshold(&mut self, new_threshold: u32) {
        self.proximity_threshold = new_threshold;
    }
}
