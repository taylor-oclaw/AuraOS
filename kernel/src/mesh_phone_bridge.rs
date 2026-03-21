extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mesh_phone_bridge_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mesh_phone_bridge_exit() {
    // Cleanup logic for the module
}

pub struct MeshPhoneBridge {
    devices: Vec<String>,
    messages: Vec<String>,
}

impl MeshPhoneBridge {
    pub fn new() -> Self {
        MeshPhoneBridge {
            devices: Vec::new(),
            messages: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device_id: &str) {
        self.devices.push(String::from(device_id));
    }

    pub fn remove_device(&mut self, device_id: &str) -> bool {
        if let Some(index) = self.devices.iter().position(|d| d == device_id) {
            self.devices.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_devices(&self) -> Vec<String> {
        self.devices.clone()
    }

    pub fn send_message(&mut self, message: &str) {
        self.messages.push(String::from(message));
    }

    pub fn receive_messages(&mut self) -> Vec<String> {
        let messages = self.messages.clone();
        self.messages.clear();
        messages
    }
}
