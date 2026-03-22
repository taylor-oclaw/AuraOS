extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct FamilyHubSyncEngine {
    devices: Vec<String>,
    sync_status: bool,
}

impl FamilyHubSyncEngine {
    pub fn new() -> Self {
        FamilyHubSyncEngine {
            devices: Vec::new(),
            sync_status: false,
        }
    }

    pub fn add_device(&mut self, device_name: &str) {
        self.devices.push(String::from(device_name));
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

    pub fn start_sync(&mut self) {
        self.sync_status = true;
        // Simulate sync process
        for device in &self.devices {
            // Placeholder logic for syncing with each device
            println!("Syncing with {}", device);
        }
    }

    pub fn stop_sync(&mut self) {
        self.sync_status = false;
    }

    pub fn is_syncing(&self) -> bool {
        self.sync_status
    }
}