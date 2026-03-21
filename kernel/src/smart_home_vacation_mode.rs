extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod smart_home_vacation_mode {
    use super::*;

    pub struct VacationMode {
        enabled: bool,
        devices: Vec<String>,
        schedule: String,
        temperature: u8,
        security_level: u8,
    }

    impl VacationMode {
        pub fn new() -> Self {
            VacationMode {
                enabled: false,
                devices: Vec::new(),
                schedule: String::from("08:00-17:00"),
                temperature: 22,
                security_level: 3,
            }
        }

        pub fn enable(&mut self) {
            self.enabled = true;
            // Logic to activate vacation mode
        }

        pub fn disable(&mut self) {
            self.enabled = false;
            // Logic to deactivate vacation mode
        }

        pub fn add_device(&mut self, device: &str) {
            self.devices.push(String::from(device));
            // Logic to add a device to the list
        }

        pub fn remove_device(&mut self, device: &str) {
            if let Some(pos) = self.devices.iter().position(|d| d == device) {
                self.devices.remove(pos);
            }
            // Logic to remove a device from the list
        }

        pub fn set_schedule(&mut self, schedule: &str) {
            self.schedule = String::from(schedule);
            // Logic to update the vacation schedule
        }

        pub fn set_temperature(&mut self, temperature: u8) {
            self.temperature = temperature;
            // Logic to set the desired temperature
        }

        pub fn set_security_level(&mut self, level: u8) {
            self.security_level = level;
            // Logic to set the security level
        }
    }
}
