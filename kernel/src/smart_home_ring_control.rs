extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    let ring_control = SmartHomeRingControl::new();
    ring_control.add_device("Living Room Light");
    ring_control.add_device("Kitchen Light");
    ring_control.turn_on_device("Living Room Light");
    ring_control.turn_off_device("Kitchen Light");
}

pub struct SmartHomeRingControl {
    devices: Vec<String>,
    device_status: Vec<bool>,
}

impl SmartHomeRingControl {
    pub fn new() -> Self {
        SmartHomeRingControl {
            devices: Vec::new(),
            device_status: Vec::new(),
        }
    }

    pub fn add_device(&mut self, name: &str) {
        self.devices.push(String::from(name));
        self.device_status.push(false);
    }

    pub fn turn_on_device(&mut self, name: &str) -> bool {
        if let Some(index) = self.devices.iter().position(|d| d == name) {
            self.device_status[index] = true;
            return true;
        }
        false
    }

    pub fn turn_off_device(&mut self, name: &str) -> bool {
        if let Some(index) = self.devices.iter().position(|d| d == name) {
            self.device_status[index] = false;
            return true;
        }
        false
    }

    pub fn get_device_status(&self) -> Vec<(String, bool)> {
        self.devices.iter()
            .zip(self.device_status.iter())
            .map(|(device, status)| (device.clone(), *status))
            .collect()
    }

    pub fn list_devices(&self) -> Vec<String> {
        self.devices.clone()
    }
}
