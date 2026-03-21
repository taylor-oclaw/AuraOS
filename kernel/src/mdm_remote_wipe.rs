extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mdm_remote_wipe_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mdm_remote_wipe_exit() {
    // Cleanup logic for the module
}

pub struct MDMRemoteWipe {
    devices: Vec<String>,
    status: String,
}

impl MDMRemoteWipe {
    pub fn new() -> Self {
        MDMRemoteWipe {
            devices: Vec::new(),
            status: String::from("Idle"),
        }
    }

    pub fn add_device(&mut self, device_id: &str) {
        self.devices.push(device_id.to_string());
    }

    pub fn remove_device(&mut self, device_id: &str) -> bool {
        if let Some(index) = self.devices.iter().position(|d| d == device_id) {
            self.devices.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_devices(&self) -> Vec<String> {
        self.devices.clone()
    }

    pub fn wipe_device(&mut self, device_id: &str) -> bool {
        if let Some(index) = self.devices.iter().position(|d| d == device_id) {
            // Simulate wiping the device
            self.devices.remove(index);
            self.status = String::from("Wiping");
            true
        } else {
            false
        }
    }

    pub fn get_status(&self) -> String {
        self.status.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mdm_remote_wipe() {
        let mut mdm = MDMRemoteWipe::new();
        assert_eq!(mdm.get_status(), "Idle");

        mdm.add_device("device1");
        mdm.add_device("device2");
        assert_eq!(mdm.list_devices(), vec![String::from("device1"), String::from("device2")]);

        assert!(mdm.wipe_device("device1"));
        assert_eq!(mdm.get_status(), "Wiping");
        assert_eq!(mdm.list_devices(), vec![String::from("device2")]);

        assert!(!mdm.wipe_device("device3"));
        assert_eq!(mdm.list_devices(), vec![String::from("device2")]);

        mdm.remove_device("device2");
        assert_eq!(mdm.list_devices(), Vec::<String>::new());
    }
}
