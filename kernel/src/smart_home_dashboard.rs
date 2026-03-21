extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn smart_home_dashboard_init() {
    // Initialization logic for the smart home dashboard module
}

pub extern "C" fn smart_home_dashboard_exit() {
    // Cleanup logic for the smart home dashboard module
}

pub struct SmartHomeDashboard {
    devices: Vec<String>,
    status: String,
}

impl SmartHomeDashboard {
    pub fn new() -> Self {
        SmartHomeDashboard {
            devices: Vec::new(),
            status: String::from("Initialized"),
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

    pub fn update_status(&mut self, new_status: &str) {
        self.status = String::from(new_status);
    }

    pub fn get_status(&self) -> String {
        self.status.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_home_dashboard() {
        let mut dashboard = SmartHomeDashboard::new();
        assert_eq!(dashboard.get_status(), "Initialized");

        dashboard.add_device("Light");
        dashboard.add_device("Thermostat");
        assert_eq!(dashboard.list_devices(), vec![String::from("Light"), String::from("Thermostat")]);

        assert!(dashboard.remove_device("Light"));
        assert_eq!(dashboard.list_devices(), vec![String::from("Thermostat")]);
        assert!(!dashboard.remove_device("Light"));

        dashboard.update_status("Active");
        assert_eq!(dashboard.get_status(), "Active");
    }
}
