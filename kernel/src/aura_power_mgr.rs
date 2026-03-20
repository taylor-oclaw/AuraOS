extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_ffi_init() {
    // Initialize the module
}

pub extern "C" fn rust_ffi_exit() {
    // Clean up the module
}

struct AuraPowerMgr {
    devices: Vec<String>,
    power_state: bool,
}

impl AuraPowerMgr {
    pub fn new() -> Self {
        AuraPowerMgr {
            devices: Vec::new(),
            power_state: false,
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

    pub fn set_power_state(&mut self, state: bool) {
        self.power_state = state;
    }

    pub fn get_power_state(&self) -> bool {
        self.power_state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aura_power_mgr() {
        let mut mgr = AuraPowerMgr::new();
        assert_eq!(mgr.list_devices(), Vec::<String>::new());
        assert!(!mgr.get_power_state());

        mgr.add_device("GPU");
        mgr.add_device("CPU");
        assert_eq!(mgr.list_devices(), vec![String::from("GPU"), String::from("CPU")]);

        assert!(mgr.remove_device("GPU"));
        assert_eq!(mgr.list_devices(), vec![String::from("CPU")]);

        assert!(!mgr.remove_device("GPU")); // Trying to remove a non-existent device
        assert_eq!(mgr.list_devices(), vec![String::from("CPU")]);

        mgr.set_power_state(true);
        assert!(mgr.get_power_state());

        mgr.set_power_state(false);
        assert!(!mgr.get_power_state());
    }
}
