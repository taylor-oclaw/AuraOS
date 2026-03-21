extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct CrossDeviceDragDrop {
    devices: Vec<String>,
    active_drag: Option<(String, String)>, // (source_device, data)
}

impl CrossDeviceDragDrop {
    pub fn new(devices: Vec<String>) -> Self {
        CrossDeviceDragDrop {
            devices,
            active_drag: None,
        }
    }

    pub fn add_device(&mut self, device_name: &str) {
        if !self.devices.contains(&device_name.to_string()) {
            self.devices.push(device_name.to_string());
        }
    }

    pub fn remove_device(&mut self, device_name: &str) {
        self.devices.retain(|d| d != device_name);
        if let Some((source, _)) = &self.active_drag {
            if source == device_name {
                self.active_drag = None;
            }
        }
    }

    pub fn start_drag(&mut self, source_device: &str, data: &str) -> bool {
        if self.devices.contains(&source_device.to_string()) && self.active_drag.is_none() {
            self.active_drag = Some((source_device.to_string(), data.to_string()));
            true
        } else {
            false
        }
    }

    pub fn drop_data(&mut self, target_device: &str) -> Option<String> {
        if let Some((source, data)) = self.active_drag.take() {
            if self.devices.contains(&target_device.to_string()) && source != target_device {
                // Simulate data transfer logic here
                return Some(data);
            }
        }
        None
    }

    pub fn get_active_drag(&self) -> Option<&(String, String)> {
        self.active_drag.as_ref()
    }
}
