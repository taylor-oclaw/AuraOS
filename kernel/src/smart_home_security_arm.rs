extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    let mut security_system = SmartHomeSecurityArm::new();
    security_system.add_device("door_sensor");
    security_system.add_device("motion_sensor");
    security_system.enable_arm_mode();
    if security_system.is_armed() {
    }
    if security_system.check_device_status("door_sensor") {
    }
    let devices = security_system.list_devices();
    for device in devices.iter() {
    }
}

pub struct SmartHomeSecurityArm {
    devices: Vec<String>,
    armed: bool,
}

impl SmartHomeSecurityArm {
    pub fn new() -> Self {
        SmartHomeSecurityArm {
            devices: Vec::new(),
            armed: false,
        }
    }

    pub fn add_device(&mut self, device_name: &str) {
        self.devices.push(String::from(device_name));
    }

    pub fn remove_device(&mut self, device_name: &str) {
        if let Some(index) = self.devices.iter().position(|d| d == device_name) {
            self.devices.remove(index);
        }
    }

    pub fn enable_arm_mode(&mut self) {
        self.armed = true;
    }

    pub fn disable_arm_mode(&mut self) {
        self.armed = false;
    }

    pub fn is_armed(&self) -> bool {
        self.armed
    }

    pub fn check_device_status(&self, device_name: &str) -> bool {
        self.devices.contains(&String::from(device_name))
    }

    pub fn list_devices(&self) -> Vec<String> {
        self.devices.clone()
    }
}
