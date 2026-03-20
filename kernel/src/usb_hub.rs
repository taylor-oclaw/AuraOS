extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn usb_hub_init() {
    // Initialization logic for the USB hub module
}

#[no_mangle]
pub extern "C" fn usb_hub_exit() {
    // Cleanup logic for the USB hub module
}

pub struct UsbHub {
    devices: Vec<String>,
    active_port: usize,
}

impl UsbHub {
    pub fn new() -> Self {
        UsbHub {
            devices: Vec::new(),
            active_port: 0,
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

    pub fn set_active_port(&mut self, port: usize) -> bool {
        if port < self.devices.len() {
            self.active_port = port;
            true
        } else {
            false
        }
    }

    pub fn get_active_device(&self) -> Option<&String> {
        self.devices.get(self.active_port)
    }
}
