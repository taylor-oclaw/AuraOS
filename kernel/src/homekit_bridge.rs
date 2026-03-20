extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    let bridge = HomekitBridge::new();
    bridge.initialize();
    bridge.add_device(String::from("Light"));
    bridge.add_device(String::from("Thermostat"));
    bridge.list_devices();
    bridge.remove_device(String::from("Light"));
    bridge.list_devices();
}

pub struct HomekitBridge {
    devices: Vec<String>,
}

impl HomekitBridge {
    pub fn new() -> Self {
        HomekitBridge {
            devices: Vec::new(),
        }
    }

    pub fn initialize(&mut self) {
        println!("HomeKit Bridge initialized.");
    }

    pub fn add_device(&mut self, device_name: String) {
        if !self.devices.contains(&device_name) {
            self.devices.push(device_name);
            println!("Device added: {}", &self.devices.last().unwrap());
        } else {
            println!("Device already exists: {}", device_name);
        }
    }

    pub fn remove_device(&mut self, device_name: String) {
        if let Some(index) = self.devices.iter().position(|x| *x == device_name) {
            let removed_device = self.devices.remove(index);
            println!("Device removed: {}", removed_device);
        } else {
            println!("Device not found: {}", device_name);
        }
    }

    pub fn list_devices(&self) {
        if self.devices.is_empty() {
            println!("No devices connected.");
        } else {
            println!("Connected devices:");
            for device in &self.devices {
                println!("{}", device);
            }
        }
    }

    pub fn get_device_count(&self) -> usize {
        self.devices.len()
    }
}
