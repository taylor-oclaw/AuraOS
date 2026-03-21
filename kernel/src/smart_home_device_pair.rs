extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct SmartHomeDevicePair {
    device_id: String,
    device_type: String,
    is_paired: bool,
    capabilities: Vec<String>,
    status: String,
}

impl SmartHomeDevicePair {
    pub fn new(device_id: &str, device_type: &str) -> Self {
        SmartHomeDevicePair {
            device_id: String::from(device_id),
            device_type: String::from(device_type),
            is_paired: false,
            capabilities: Vec::new(),
            status: String::from("offline"),
        }
    }

    pub fn pair(&mut self) {
        if !self.is_paired {
            self.is_paired = true;
            self.status = String::from("paired");
        }
    }

    pub fn unpair(&mut self) {
        if self.is_paired {
            self.is_paired = false;
            self.status = String::from("unpaired");
        }
    }

    pub fn add_capability(&mut self, capability: &str) {
        if !self.capabilities.contains(&String::from(capability)) {
            self.capabilities.push(String::from(capability));
        }
    }

    pub fn remove_capability(&mut self, capability: &str) {
        self.capabilities.retain(|c| c != capability);
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_pairing() {
        let mut device = SmartHomeDevicePair::new("123", "light");
        assert_eq!(device.get_status(), "offline");

        device.pair();
        assert_eq!(device.get_status(), "paired");

        device.unpair();
        assert_eq!(device.get_status(), "unpaired");
    }

    #[test]
    fn test_device_capabilities() {
        let mut device = SmartHomeDevicePair::new("456", "thermostat");
        assert_eq!(device.capabilities.len(), 0);

        device.add_capability("temperature_control");
        assert_eq!(device.capabilities.len(), 1);
        assert!(device.capabilities.contains(&String::from("temperature_control")));

        device.remove_capability("temperature_control");
        assert_eq!(device.capabilities.len(), 0);
    }
}
