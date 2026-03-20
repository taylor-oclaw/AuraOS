extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn zwave_handler_init() {
    // Initialization logic for the Z-Wave handler module
}

pub extern "C" fn zwave_handler_exit() {
    // Cleanup logic for the Z-Wave handler module
}

pub struct ZWaveHandler {
    devices: Vec<String>,
    messages: Vec<String>,
}

impl ZWaveHandler {
    pub fn new() -> Self {
        ZWaveHandler {
            devices: Vec::new(),
            messages: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device_name: &str) {
        let name = String::from(device_name);
        self.devices.push(name);
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

    pub fn send_message(&mut self, message: &str) {
        let msg = String::from(message);
        self.messages.push(msg);
    }

    pub fn get_messages(&self) -> Vec<String> {
        self.messages.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zwave_handler() {
        let mut handler = ZWaveHandler::new();
        assert_eq!(handler.list_devices().len(), 0);

        handler.add_device("Device1");
        handler.add_device("Device2");
        assert_eq!(handler.list_devices().len(), 2);
        assert!(handler.remove_device("Device1"));
        assert!(!handler.remove_device("Device3"));
        assert_eq!(handler.list_devices().len(), 1);

        handler.send_message("Hello, Device2!");
        assert_eq!(handler.get_messages().len(), 1);
    }
}
