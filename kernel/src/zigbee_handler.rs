extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn zigbee_handler_init() {
    // Initialization logic for the Zigbee handler module
}

#[no_mangle]
pub extern "C" fn zigbee_handler_exit() {
    // Cleanup logic for the Zigbee handler module
}

pub struct ZigbeeHandler {
    devices: Vec<String>,
    messages: Vec<String>,
}

impl ZigbeeHandler {
    pub fn new() -> Self {
        ZigbeeHandler {
            devices: Vec::new(),
            messages: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device_id: &str) {
        self.devices.push(device_id.to_string());
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
        self.messages.push(message.to_string());
    }

    pub fn receive_messages(&mut self) -> Vec<String> {
        let messages = self.messages.clone();
        self.messages.clear();
        messages
    }
}
