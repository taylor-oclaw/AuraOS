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
        if !self.devices.contains(&device_name.to_string()) {
            self.devices.push(device_name.to_string());
        }
    }

    pub fn remove_device(&mut self, device_name: &str) {
        self.devices.retain(|d| d != device_name);
    }

    pub fn list_devices(&self) -> Vec<String> {
        self.devices.clone()
    }

    pub fn start_sync(&mut self) {
        if !self.sync_status {
            // Simulate sync process
            self.sync_status = true;
            // Additional logic for starting sync can be added here
        }
    }

    pub fn stop_sync(&mut self) {
        if self.sync_status {
            // Simulate stopping sync process
            self.sync_status = false;
            // Additional logic for stopping sync can be added here
        }
    }
}
