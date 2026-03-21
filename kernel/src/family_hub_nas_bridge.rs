extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn family_hub_nas_bridge_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn family_hub_nas_bridge_exit() {
    // Cleanup logic for the module
}

pub struct FamilyHubNasBridge {
    devices: Vec<String>,
    connected_users: Vec<String>,
    shared_folders: Vec<String>,
    logs: Vec<String>,
}

impl FamilyHubNasBridge {
    pub fn new() -> Self {
        FamilyHubNasBridge {
            devices: Vec::new(),
            connected_users: Vec::new(),
            shared_folders: Vec::new(),
            logs: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device_name: &str) {
        self.devices.push(device_name.to_string());
        self.log(format!("Device added: {}", device_name));
    }

    pub fn remove_device(&mut self, device_name: &str) -> bool {
        if let Some(index) = self.devices.iter().position(|d| d == device_name) {
            self.devices.remove(index);
            self.log(format!("Device removed: {}", device_name));
            true
        } else {
            false
        }
    }

    pub fn connect_user(&mut self, user_name: &str) -> bool {
        if !self.connected_users.contains(&user_name.to_string()) {
            self.connected_users.push(user_name.to_string());
            self.log(format!("User connected: {}", user_name));
            true
        } else {
            false
        }
    }

    pub fn disconnect_user(&mut self, user_name: &str) -> bool {
        if let Some(index) = self.connected_users.iter().position(|u| u == user_name) {
            self.connected_users.remove(index);
            self.log(format!("User disconnected: {}", user_name));
            true
        } else {
            false
        }
    }

    pub fn share_folder(&mut self, folder_name: &str) -> bool {
        if !self.shared_folders.contains(&folder_name.to_string()) {
            self.shared_folders.push(folder_name.to_string());
            self.log(format!("Folder shared: {}", folder_name));
            true
        } else {
            false
        }
    }

    fn log(&mut self, message: String) {
        self.logs.push(message);
    }
}
