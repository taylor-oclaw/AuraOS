extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mesh_weekend_mode_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mesh_weekend_mode_exit() {
    // Cleanup logic for the module
}

pub struct MeshWeekendMode {
    enabled: bool,
    devices: Vec<String>,
    schedule: String,
    duration_hours: u32,
    notifications_enabled: bool,
}

impl MeshWeekendMode {
    pub fn new(enabled: bool, devices: Vec<String>, schedule: String, duration_hours: u32, notifications_enabled: bool) -> Self {
        MeshWeekendMode {
            enabled,
            devices,
            schedule,
            duration_hours,
            notifications_enabled,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn add_device(&mut self, device: String) {
        if !self.devices.contains(&device) {
            self.devices.push(device);
        }
    }

    pub fn remove_device(&mut self, device: &str) -> bool {
        let index = self.devices.iter().position(|d| d == device);
        match index {
            Some(i) => {
                self.devices.remove(i);
                true
            },
            None => false,
        }
    }

    pub fn set_schedule(&mut self, schedule: String) {
        self.schedule = schedule;
    }

    pub fn get_schedule(&self) -> &str {
        &self.schedule
    }

    pub fn set_duration_hours(&mut self, duration_hours: u32) {
        self.duration_hours = duration_hours;
    }

    pub fn get_duration_hours(&self) -> u32 {
        self.duration_hours
    }

    pub fn enable_notifications(&mut self) {
        self.notifications_enabled = true;
    }

    pub fn disable_notifications(&mut self) {
        self.notifications_enabled = false;
    }

    pub fn are_notifications_enabled(&self) -> bool {
        self.notifications_enabled
    }
}
