extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mesh_overnight_mode_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mesh_overnight_mode_exit() {
    // Cleanup logic for the module
}

pub struct MeshOvernightMode {
    enabled: bool,
    devices: Vec<String>,
    schedule: Vec<(u8, u8)>, // (start_hour, end_hour)
}

impl MeshOvernightMode {
    pub fn new() -> Self {
        MeshOvernightMode {
            enabled: false,
            devices: Vec::new(),
            schedule: Vec::new(),
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn add_device(&mut self, device_name: &str) {
        self.devices.push(String::from(device_name));
    }

    pub fn remove_device(&mut self, device_name: &str) {
        self.devices.retain(|d| d != device_name);
    }

    pub fn set_schedule(&mut self, schedule: Vec<(u8, u8)>) {
        self.schedule = schedule;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_devices(&self) -> &Vec<String> {
        &self.devices
    }

    pub fn get_schedule(&self) -> &Vec<(u8, u8)> {
        &self.schedule
    }
}
