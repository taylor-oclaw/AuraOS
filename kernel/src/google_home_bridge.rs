extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    let bridge = GoogleHomeBridge::new();
    bridge.initialize();
    bridge.add_device("Living Room Light");
    bridge.add_device("Kitchen Lamp");
    bridge.list_devices();
    bridge.remove_device("Living Room Light");
    bridge.list_devices();
}

pub struct GoogleHomeBridge {
    devices: Vec<String>,
}

impl GoogleHomeBridge {
    pub fn new() -> Self {
        GoogleHomeBridge {
            devices: Vec::new(),
        }
    }

    pub fn initialize(&mut self) {
        println!("Google Home Bridge initialized.");
    }

    pub fn add_device(&mut self, device_name: &str) {
        self.devices.push(String::from(device_name));
        println!("Device added: {}", device_name);
    }

    pub fn remove_device(&mut self, device_name: &str) {
        if let Some(index) = self.devices.iter().position(|x| x == device_name) {
            self.devices.remove(index);
            println!("Device removed: {}", device_name);
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
