extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct SmartHomeEnergyReport {
    device_name: String,
    energy_usage: Vec<u32>,
    total_energy_consumed: u32,
}

impl SmartHomeEnergyReport {
    pub fn new(device_name: &str) -> Self {
        SmartHomeEnergyReport {
            device_name: String::from(device_name),
            energy_usage: Vec::new(),
            total_energy_consumed: 0,
        }
    }

    pub fn add_energy_usage(&mut self, usage: u32) {
        self.energy_usage.push(usage);
        self.total_energy_consumed += usage;
    }

    pub fn get_device_name(&self) -> &str {
        &self.device_name
    }

    pub fn get_total_energy_consumed(&self) -> u32 {
        self.total_energy_consumed
    }

    pub fn average_energy_usage(&self) -> Option<u32> {
        if self.energy_usage.is_empty() {
            None
        } else {
            Some(self.total_energy_consumed / self.energy_usage.len() as u32)
        }
    }

    pub fn get_energy_usage_history(&self) -> &Vec<u32> {
        &self.energy_usage
    }
}
