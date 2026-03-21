extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ParentalEmergencyAlert {
    alerts: Vec<String>,
}

impl ParentalEmergencyAlert {
    pub fn new() -> Self {
        ParentalEmergencyAlert {
            alerts: Vec::new(),
        }
    }

    pub fn add_alert(&mut self, alert_message: &str) {
        self.alerts.push(String::from(alert_message));
    }

    pub fn remove_alert(&mut self, index: usize) -> Option<String> {
        if index < self.alerts.len() {
            Some(self.alerts.remove(index))
        } else {
            None
        }
    }

    pub fn get_alert(&self, index: usize) -> Option<&String> {
        self.alerts.get(index)
    }

    pub fn list_all_alerts(&self) -> &[String] {
        &self.alerts
    }

    pub fn clear_all_alerts(&mut self) {
        self.alerts.clear();
    }
}
