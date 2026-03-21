extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
#[derive(Clone, Copy, PartialEq)]
pub enum DeviceType {
    Keyboard,
    Mouse,
    Display,
    Storage,
    Network,
    Audio,
    Serial,
    USB,
    Unknown,
}

#[derive(Debug)]
#[derive(Clone, Copy, PartialEq)]
pub enum DeviceStatus {
    Active,
    Inactive,
    Error,
    Initializing,
}

#[derive(Debug)]
pub struct Device {
    id: u32,
    name: String,
    device_type: DeviceType,
    status: DeviceStatus,
    vendor_id: u16,
    device_id: u16,
}

pub struct DeviceManager {
    devices: Vec<Device>,
    next_id: u32,
}

impl DeviceManager {
    fn new() -> Self {
        DeviceManager {
            devices: Vec::new(),
            next_id: 0,
        }
    }

    fn register(&mut self, name: String, device_type: DeviceType, vendor_id: u16, device_id: u16) -> u32 {
        let id = self.next_id;
        self.devices.push(Device {
            id,
            name,
            device_type,
            status: DeviceStatus::Initializing,
            vendor_id,
            device_id,
        };
        self.next_id += 1;
        id
    }

    fn unregister(&mut self, id: u32) -> Option<Device> {
        let pos = self.devices.iter().position(|d| d.id == id);
        if let Some(index) = pos {
            Some(self.devices.remove(index))
        } else {
            None
        }
    }

    fn set_status(&mut self, id: u32, status: DeviceStatus) -> bool {
        if let Some(device) = self.devices.iter_mut().find(|d| d.id == id) {
            device.status = status;
            true
        } else {
            false
        }
    }

    fn get_device(&self, id: u32) -> Option<&Device> {
        self.devices.iter().find(|d| d.id == id)
    }

    fn find_by_type(&self, device_type: DeviceType) -> Vec<&Device> {
        self.devices.iter().filter(|d| d.device_type == device_type).collect()
    }

    fn active_count(&self) -> usize {
        self.devices.iter().filter(|d| d.status == DeviceStatus::Active).count()
    }

    fn list_all(&self) -> &[Device] {
        &self.devices
    }
)}
