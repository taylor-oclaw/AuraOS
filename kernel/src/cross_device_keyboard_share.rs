extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CrossDeviceKeyboardShare {
    device_list: Vec<String>,
    shared_buffer: String,
}

impl CrossDeviceKeyboardShare {
    pub fn new() -> Self {
        CrossDeviceKeyboardShare {
            device_list: Vec::new(),
            shared_buffer: String::new(),
        }
    }

    pub fn add_device(&mut self, device_name: &str) {
        if !self.device_list.contains(&device_name.to_string()) {
            self.device_list.push(device_name.to_string());
        }
    }

    pub fn remove_device(&mut self, device_name: &str) {
        self.device_list.retain(|d| d != device_name);
    }

    pub fn list_devices(&self) -> Vec<String> {
        self.device_list.clone()
    }

    pub fn send_key_event(&mut self, key_event: &str) {
        self.shared_buffer.push_str(key_event);
    }

    pub fn receive_key_events(&mut self) -> String {
        let events = self.shared_buffer.clone();
        self.shared_buffer.clear();
        events
    }
}
