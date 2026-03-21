extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SmartHomeCameraAlert {
    alerts: Vec<String>,
    is_active: bool,
}

impl SmartHomeCameraAlert {
    pub fn new() -> Self {
        SmartHomeCameraAlert {
            alerts: Vec::new(),
            is_active: true,
        }
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn add_alert(&mut self, message: &str) {
        if self.is_active {
            self.alerts.push(String::from(message));
        }
    }

    pub fn clear_alerts(&mut self) {
        self.alerts.clear();
    }

    pub fn get_alerts(&self) -> &[String] {
        &self.alerts
    }
}
