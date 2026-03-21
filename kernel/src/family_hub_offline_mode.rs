extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct FamilyHubOfflineMode {
    devices: Vec<String>,
    users: Vec<String>,
    settings: Vec<(String, String)>,
    logs: Vec<String>,
    status: bool,
}

impl FamilyHubOfflineMode {
    pub fn new() -> Self {
        FamilyHubOfflineMode {
            devices: Vec::new(),
            users: Vec::new(),
            settings: Vec::new(),
            logs: Vec::new(),
            status: false,
        }
    }

    pub fn add_device(&mut self, device_name: &str) {
        self.devices.push(device_name.to_string());
    }

    pub fn remove_device(&mut self, device_name: &str) {
        if let Some(index) = self.devices.iter().position(|d| d == device_name) {
            self.devices.remove(index);
        }
    }

    pub fn add_user(&mut self, user_name: &str) {
        self.users.push(user_name.to_string());
    }

    pub fn remove_user(&mut self, user_name: &str) {
        if let Some(index) = self.users.iter().position(|u| u == user_name) {
            self.users.remove(index);
        }
    }

    pub fn set_setting(&mut self, key: &str, value: &str) {
        match self.settings.iter_mut().find(|(k, _)| k == key) {
            Some(entry) => entry.1 = value.to_string(),
            None => self.settings.push((key.to_string(), value.to_string())),
        }
    }

    pub fn get_setting(&self, key: &str) -> Option<&String> {
        self.settings.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn enable_offline_mode(&mut self) {
        self.status = true;
        self.logs.push("Offline mode enabled".to_string());
    }

    pub fn disable_offline_mode(&mut self) {
        self.status = false;
        self.logs.push("Offline mode disabled".to_string());
    }

    pub fn is_offline_mode_enabled(&self) -> bool {
        self.status
    }

    pub fn log_event(&mut self, event: &str) {
        self.logs.push(event.to_string());
    }

    pub fn get_logs(&self) -> &[String] {
        &self.logs
    }
}
