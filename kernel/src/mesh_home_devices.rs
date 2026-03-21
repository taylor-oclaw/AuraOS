extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mesh_home_devices_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mesh_home_devices_exit() {
    // Cleanup logic for the module
}

pub struct DeviceManager {
    devices: Vec<String>,
}

impl DeviceManager {
    pub fn new() -> Self {
        DeviceManager {
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

    pub fn find_device(&self, device_name: &str) -> Option<&String> {
        self.devices.iter().find(|&&d| d == device_name)
    }

    pub fn count_devices(&self) -> usize {
        self.devices.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_manager() {
        let mut manager = DeviceManager::new();
        assert_eq!(manager.count_devices(), 0);

        manager.add_device("Light");
        manager.add_device("Thermostat");
        assert_eq!(manager.count_devices(), 2);
        assert_eq!(manager.list_devices(), vec!["Light".to_string(), "Thermostat".to_string()]);

        assert!(manager.find_device("Light").is_some());
        assert!(manager.find_device("Fan").is_none());

        manager.remove_device("Light");
        assert_eq!(manager.count_devices(), 1);
        assert_eq!(manager.list_devices(), vec!["Thermostat".to_string()]);
    }
}
