extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SmartHomeDeviceRegistry {
    devices: Vec<SmartHomeDevice>,
}

impl SmartHomeDeviceRegistry {
    pub fn new() -> Self {
        SmartHomeDeviceRegistry {
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device: SmartHomeDevice) {
        self.devices.push(device);
    }

    pub fn remove_device_by_id(&mut self, id: u32) -> Option<SmartHomeDevice> {
        let index = self.devices.iter().position(|d| d.id == id)?;
        Some(self.devices.remove(index))
    }

    pub fn get_device_by_id(&self, id: u32) -> Option<&SmartHomeDevice> {
        self.devices.iter().find(|&d| d.id == id)
    }

    pub fn list_all_devices(&self) -> &[SmartHomeDevice] {
        &self.devices
    }

    pub fn update_device_name(&mut self, id: u32, new_name: String) -> bool {
        if let Some(device) = self.get_device_by_id(id) {
            device.name = new_name;
            true
        } else {
            false
        }
    }
}

pub struct SmartHomeDevice {
    pub id: u32,
    pub name: String,
    pub device_type: String,
    pub location: String,
    pub status: DeviceStatus,
}

#[derive(Clone, Copy)]
pub enum DeviceStatus {
    On,
    Off,
    Standby,
}

impl SmartHomeDevice {
    pub fn new(id: u32, name: String, device_type: String, location: String) -> Self {
        SmartHomeDevice {
            id,
            name,
            device_type,
            location,
            status: DeviceStatus::Off,
        }
    }

    pub fn turn_on(&mut self) {
        self.status = DeviceStatus::On;
    }

    pub fn turn_off(&mut self) {
        self.status = DeviceStatus::Off;
    }

    pub fn set_standby(&mut self) {
        self.status = DeviceStatus::Standby;
    }
}
