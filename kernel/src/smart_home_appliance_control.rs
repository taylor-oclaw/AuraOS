extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct SmartHomeApplianceControl {
    devices: Vec<String>,
}

impl SmartHomeApplianceControl {
    pub fn new() -> Self {
        SmartHomeApplianceControl {
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device_name: &str) {
        self.devices.push(String::from(device_name));
    }

    pub fn remove_device(&mut self, device_name: &str) {
        if let Some(index) = self.devices.iter().position(|d| d == device_name) {
            self.devices.remove(index);
        }
    }

    pub fn list_devices(&self) -> Vec<String> {
        self.devices.clone()
    }

    pub fn has_device(&self, device_name: &str) -> bool {
        self.devices.contains(&String::from(device_name))
    }

    pub fn count_devices(&self) -> usize {
        self.devices.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_home_appliance_control() {
        let mut control = SmartHomeApplianceControl::new();
        assert_eq!(control.count_devices(), 0);

        control.add_device("Light");
        assert_eq!(control.count_devices(), 1);
        assert!(control.has_device("Light"));

        control.add_device("Thermostat");
        assert_eq!(control.count_devices(), 2);
        assert!(control.has_device("Thermostat"));

        let devices = control.list_devices();
        assert_eq!(devices, vec![String::from("Light"), String::from("Thermostat")]);

        control.remove_device("Light");
        assert_eq!(control.count_devices(), 1);
        assert!(!control.has_device("Light"));
    }
}
