extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SmartHomeTriggerSunrise {
    sunrise_time: u32,
    devices_to_trigger: Vec<String>,
    is_active: bool,
}

impl SmartHomeTriggerSunrise {
    pub fn new(sunrise_time: u32) -> Self {
        SmartHomeTriggerSunrise {
            sunrise_time,
            devices_to_trigger: Vec::new(),
            is_active: false,
        }
    }

    pub fn add_device(&mut self, device_name: &str) {
        self.devices_to_trigger.push(device_name.to_string());
    }

    pub fn remove_device(&mut self, device_name: &str) {
        self.devices_to_trigger.retain(|d| d != device_name);
    }

    pub fn get_devices(&self) -> Vec<String> {
        self.devices_to_trigger.clone()
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn is_triggered(&self, current_time: u32) -> bool {
        self.is_active && current_time == self.sunrise_time
    }
}
