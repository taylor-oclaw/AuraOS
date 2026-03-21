extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut security_alert = SmartHomeSecurityAlert::new();
    security_alert.add_device("door_sensor");
    security_alert.add_device("motion_sensor");
    security_alert.enable_alerts(true);
    security_alert.trigger_alert("door_sensor", "Unauthorized access detected!");
    println!("Alerts enabled: {}", security_alert.are_alerts_enabled());
}

pub struct SmartHomeSecurityAlert {
    devices: Vec<String>,
    alerts_enabled: bool,
}

impl SmartHomeSecurityAlert {
    pub fn new() -> Self {
        SmartHomeSecurityAlert {
            devices: Vec::new(),
            alerts_enabled: false,
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

    pub fn enable_alerts(&mut self, enabled: bool) {
        self.alerts_enabled = enabled;
    }

    pub fn are_alerts_enabled(&self) -> bool {
        self.alerts_enabled
    }

    pub fn trigger_alert(&self, device_name: &str, message: &str) {
        if self.alerts_enabled && self.devices.contains(&String::from(device_name)) {
            // Simulate sending an alert
            println!("Alert from {}: {}", device_name, message);
        }
    }
}
