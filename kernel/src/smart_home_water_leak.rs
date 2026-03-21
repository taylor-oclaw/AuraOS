extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct SmartHomeWaterLeak {
    sensor_id: u32,
    location: String,
    is_leaking: bool,
    alerts_sent: u32,
    last_checked: u64, // Unix timestamp
}

impl SmartHomeWaterLeak {
    pub fn new(sensor_id: u32, location: &str) -> Self {
        SmartHomeWaterLeak {
            sensor_id,
            location: String::from(location),
            is_leaking: false,
            alerts_sent: 0,
            last_checked: 0,
        }
    }

    pub fn check_for_leak(&mut self, current_time: u64) -> bool {
        // Simulate a leak detection logic
        let detected = /* some logic to detect water leak */;
        if detected {
            self.is_leaking = true;
            self.last_checked = current_time;
        }
        detected
    }

    pub fn send_alert(&mut self) {
        if self.is_leaking && self.alerts_sent == 0 {
            // Simulate sending an alert
            /* some logic to send an alert */;
            self.alerts_sent += 1;
        }
    }

    pub fn reset_alerts(&mut self) {
        self.alerts_sent = 0;
    }

    pub fn get_sensor_id(&self) -> u32 {
        self.sensor_id
    }

    pub fn get_location(&self) -> &str {
        &self.location
    }
}
