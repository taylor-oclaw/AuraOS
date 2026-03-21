extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct Device {
    pub name: String,
    pub vendor: String,
    pub model: String,
    pub serial_number: String,
}

impl Device {
    pub fn new(name: &str, vendor: &str, model: &str, serial_number: &str) -> Self {
        Device {
            name: String::from(name),
            vendor: String::from(vendor),
            model: String::from(model),
            serial_number: String::from(serial_number),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_vendor(&self) -> &str {
        &self.vendor
    }

    pub fn get_model(&self) -> &str {
        &self.model
    }

    pub fn get_serial_number(&self) -> &str {
        &self.serial_number
    }
}

pub struct DeviceInventory {
    devices: Vec<Device>,
}

impl DeviceInventory {
    pub fn new() -> Self {
        DeviceInventory {
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device: Device) {
        self.devices.push(device);
    }

    pub fn remove_device_by_serial(&mut self, serial_number: &str) {
        self.devices.retain(|d| d.serial_number != serial_number);
    }

    pub fn get_device_count(&self) -> usize {
        self.devices.len()
    }

    pub fn list_devices(&self) -> Vec<&Device> {
        self.devices.iter().collect()
    }
}
