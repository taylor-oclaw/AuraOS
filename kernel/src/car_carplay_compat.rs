extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn car_carplay_compat_init() {
}

pub extern "C" fn car_carplay_compat_exit() {
}

pub struct CarCarplayCompat {
    devices: Vec<String>,
    connected: bool,
}

impl CarCarplayCompat {
    pub fn new() -> Self {
        CarCarplayCompat {
            devices: Vec::new(),
            connected: false,
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

    pub fn connect(&mut self) {
        if !self.connected {
            self.connected = true;
        }
    }

    pub fn disconnect(&mut self) {
        if self.connected {
            self.connected = false;
        }
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }
}
