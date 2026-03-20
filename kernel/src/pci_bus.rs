extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PciBus {
    devices: Vec<PciDevice>,
}

impl PciBus {
    pub fn new() -> Self {
        PciBus {
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device: PciDevice) {
        self.devices.push(device);
    }

    pub fn get_device_count(&self) -> usize {
        self.devices.len()
    }

    pub fn find_device_by_id(&self, vendor_id: u16, device_id: u16) -> Option<&PciDevice> {
        self.devices.iter().find(|&d| d.vendor_id == vendor_id && d.device_id == device_id)
    }

    pub fn list_devices(&self) -> Vec<String> {
        let mut device_list = Vec::new();
        for device in &self.devices {
            device_list.push(String::from("info"));
        }
        device_list
    }

    pub fn remove_device(&mut self, vendor_id: u16, device_id: u16) -> Option<PciDevice> {
        let index = self.devices.iter().position(|d| d.vendor_id == vendor_id && d.device_id == device_id)?;
        Some(self.devices.remove(index))
    }
}

pub struct PciDevice {
    pub vendor_id: u16,
    pub device_id: u16,
    // Add more fields as necessary
}
