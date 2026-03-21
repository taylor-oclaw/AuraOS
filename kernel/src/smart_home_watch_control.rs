extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut watch_control = SmartHomeWatchControl::new();
    watch_control.add_device("Living Room".to_string(), "Smart Watch 1".to_string());
    watch_control.add_device("Bedroom".to_string(), "Smart Watch 2".to_string());

    if watch_control.is_device_connected("Smart Watch 1") {
        watch_control.send_notification("Smart Watch 1", "Meeting at 3 PM");
    }

    let devices = watch_control.list_devices();
    for device in devices.iter() {
    }
}

pub struct SmartHomeWatchControl {
    devices: Vec<(String, String)>,
}

impl SmartHomeWatchControl {
    pub fn new() -> Self {
        SmartHomeWatchControl {
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, location: String, device_name: String) {
        self.devices.push((location, device_name));
    }

    pub fn remove_device(&mut self, device_name: &str) -> bool {
        let position = self.devices.iter().position(|(_, name)| name == device_name);
        if let Some(pos) = position {
            self.devices.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn is_device_connected(&self, device_name: &str) -> bool {
        self.devices.iter().any(|(_, name)| name == device_name)
    }

    pub fn send_notification(&self, device_name: &str, message: &str) {
        if let Some((_, name)) = self.devices.iter().find(|(_, name)| name == device_name) {
        }
    }

    pub fn list_devices(&self) -> Vec<String> {
        self.devices.iter().map(|(location, name)| String::from("info")).collect()
    }
}
