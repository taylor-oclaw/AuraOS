extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct RaidManager {
    devices: Vec<String>,
    raid_level: u8,
}

impl RaidManager {
    pub fn new(devices: Vec<String>, raid_level: u8) -> Self {
        RaidManager { devices, raid_level }
    }

    pub fn add_device(&mut self, device: String) {
        self.devices.push(device);
    }

    pub fn remove_device(&mut self, device_name: &str) -> bool {
        if let Some(index) = self.devices.iter().position(|d| d == device_name) {
            self.devices.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_devices(&self) -> Vec<String> {
        self.devices.clone()
    }

    pub fn set_raid_level(&mut self, raid_level: u8) {
        self.raid_level = raid_level;
    }

    pub fn get_raid_level(&self) -> u8 {
        self.raid_level
    }
}
