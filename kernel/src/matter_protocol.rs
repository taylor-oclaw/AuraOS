extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct MatterProtocol {
    version: String,
    features: Vec<String>,
    connected_devices: Vec<String>,
}

impl MatterProtocol {
    pub fn new(version: &str) -> Self {
        MatterProtocol {
            version: String::from(version),
            features: Vec::new(),
            connected_devices: Vec::new(),
        }
    }

    pub fn add_feature(&mut self, feature: &str) {
        self.features.push(String::from(feature));
    }

    pub fn remove_feature(&mut self, feature: &str) -> bool {
        if let Some(index) = self.features.iter().position(|f| f == feature) {
            self.features.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_features(&self) -> Vec<String> {
        self.features.clone()
    }

    pub fn connect_device(&mut self, device_id: &str) {
        self.connected_devices.push(String::from(device_id));
    }

    pub fn disconnect_device(&mut self, device_id: &str) -> bool {
        if let Some(index) = self.connected_devices.iter().position(|d| d == device_id) {
            self.connected_devices.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_connected_devices(&self) -> Vec<String> {
        self.connected_devices.clone()
    }
}
