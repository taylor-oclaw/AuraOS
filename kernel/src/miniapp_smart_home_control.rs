extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    let mut control = SmartHomeControl::new();
    control.add_device("Living Room Light".to_string(), DeviceType::Light);
    control.add_device("Kitchen Appliance".to_string(), DeviceType::Appliance);

    control.turn_on("Living Room Light");
    control.turn_off("Kitchen Appliance");

    if let Some(status) = control.get_status("Living Room Light") {
    }

    control.list_devices();
}

pub struct SmartHomeControl {
    devices: Vec<Device>,
}

impl SmartHomeControl {
    pub fn new() -> Self {
        SmartHomeControl {
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, name: String, device_type: DeviceType) {
        let device = Device {
            name,
            device_type,
            is_on: false,
        };
        self.devices.push(device);
    }

    pub fn turn_on(&mut self, name: &str) {
        if let Some(device) = self.devices.iter_mut().find(|d| d.name == name) {
            device.is_on = true;
        }
    }

    pub fn turn_off(&mut self, name: &str) {
        if let Some(device) = self.devices.iter_mut().find(|d| d.name == name) {
            device.is_on = false;
        }
    }

    pub fn get_status(&self, name: &str) -> Option<String> {
        self.devices
            .iter()
            .find(|d| d.name == name)
            .map(|d| if d.is_on { "ON".to_string() } else { "OFF".to_string() })
    }

    pub fn list_devices(&self) {
        for device in &self.devices {
        }
    }
}

#[derive(Debug)]
pub enum DeviceType {
    Light,
    Appliance,
    Thermostat,
    SecurityCamera,
    DoorLock,
}

#[derive(Debug)]
struct Device {
    name: String,
    device_type: DeviceType,
    is_on: bool,
}
