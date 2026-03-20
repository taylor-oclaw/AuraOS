extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn asf_adapter_crucible_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn asf_adapter_crucible_exit() {
    // Cleanup logic for the module
}

pub struct CrucibleAdapter {
    devices: Vec<String>,
    config: String,
}

impl CrucibleAdapter {
    pub fn new(config: &str) -> Self {
        CrucibleAdapter {
            devices: Vec::new(),
            config: String::from(config),
        }
    }

    pub fn add_device(&mut self, device_name: &str) {
        self.devices.push(String::from(device_name));
    }

    pub fn remove_device(&mut self, device_name: &str) -> bool {
        if let Some(index) = self.devices.iter().position(|d| d == device_name) {
            self.devices.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_devices(&self) -> Vec<String> {
        self.devices.clone()
    }

    pub fn get_config(&self) -> String {
        self.config.clone()
    }

    pub fn set_config(&mut self, new_config: &str) {
        self.config = String::from(new_config);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crucible_adapter() {
        let mut adapter = CrucibleAdapter::new("default_config");
        assert_eq!(adapter.get_config(), "default_config");

        adapter.add_device("device1");
        adapter.add_device("device2");
        assert_eq!(adapter.list_devices(), vec![String::from("device1"), String::from("device2")]);

        assert!(adapter.remove_device("device1"));
        assert!(!adapter.remove_device("device3"));

        adapter.set_config("new_config");
        assert_eq!(adapter.get_config(), "new_config");
    }
}
