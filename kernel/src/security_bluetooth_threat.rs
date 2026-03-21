extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn security_bluetooth_threat_init() {
    // Initialization logic for the module
}

pub extern "C" fn security_bluetooth_threat_exit() {
    // Cleanup logic for the module
}

pub struct BluetoothThreatDetector {
    threats: Vec<String>,
    allowed_devices: Vec<String>,
}

impl BluetoothThreatDetector {
    pub fn new() -> Self {
        BluetoothThreatDetector {
            threats: Vec::new(),
            allowed_devices: Vec::new(),
        }
    }

    pub fn add_threat(&mut self, threat: String) {
        self.threats.push(threat);
    }

    pub fn remove_threat(&mut self, threat: &str) -> bool {
        if let Some(index) = self.threats.iter().position(|t| t == threat) {
            self.threats.remove(index);
            true
        } else {
            false
        }
    }

    pub fn add_allowed_device(&mut self, device: String) {
        self.allowed_devices.push(device);
    }

    pub fn remove_allowed_device(&mut self, device: &str) -> bool {
        if let Some(index) = self.allowed_devices.iter().position(|d| d == device) {
            self.allowed_devices.remove(index);
            true
        } else {
            false
        }
    }

    pub fn is_threat(&self, device: &str) -> bool {
        !self.allowed_devices.contains(device) && self.threats.contains(&device.to_string())
    }
}
