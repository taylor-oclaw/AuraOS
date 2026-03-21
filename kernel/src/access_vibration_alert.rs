extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AccessVibrationAlert {
    alerts: Vec<String>,
    enabled: bool,
}

impl AccessVibrationAlert {
    pub fn new() -> Self {
        AccessVibrationAlert {
            alerts: Vec::new(),
            enabled: true,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn add_alert(&mut self, alert: String) {
        if self.enabled {
            self.alerts.push(alert);
        }
    }

    pub fn get_alerts(&self) -> &[String] {
        &self.alerts
    }

    pub fn clear_alerts(&mut self) {
        self.alerts.clear();
    }
}
