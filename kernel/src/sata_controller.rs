extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SATAController {
    // Example fields for a SATA controller
    port_count: u8,
    devices: Vec<String>,
}

impl SATAController {
    pub fn new(port_count: u8) -> Self {
        SATAController {
            port_count,
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device_name: &str) {
        if self.devices.len() < self.port_count as usize {
            self.devices.push(String::from(device_name));
        } else {
            // Handle error: No available ports
        }
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

    pub fn get_device_count(&self) -> usize {
        self.devices.len()
    }

    pub fn is_port_available(&self, port: u8) -> bool {
        if port < self.port_count {
            !self.devices.iter().any(|d| d.starts_with(&String::from("info")))
        } else {
            false
        }
    }
}
