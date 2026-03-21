extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct FamilyHubHomeServer {
    devices: Vec<String>,
    users: Vec<String>,
    settings: Vec<(String, String)>,
}

impl FamilyHubHomeServer {
    pub fn new() -> Self {
        FamilyHubHomeServer {
            devices: Vec::new(),
            users: Vec::new(),
            settings: Vec::new(),
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

    pub fn add_user(&mut self, user_name: &str) {
        self.users.push(String::from(user_name));
    }

    pub fn remove_user(&mut self, user_name: &str) -> bool {
        if let Some(index) = self.users.iter().position(|u| u == user_name) {
            self.users.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_users(&self) -> Vec<String> {
        self.users.clone()
    }

    pub fn set_setting(&mut self, key: &str, value: &str) {
        if let Some(index) = self.settings.iter().position(|s| s.0 == key) {
            self.settings[index] = (String::from(key), String::from(value));
        } else {
            self.settings.push((String::from(key), String::from(value)));
        }
    }

    pub fn get_setting(&self, key: &str) -> Option<String> {
        self.settings.iter().find(|s| s.0 == key).map(|s| s.1.clone())
    }

    pub fn list_settings(&self) -> Vec<(String, String)> {
        self.settings.clone()
    }
}
