extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AccessVisualAlertSound {
    alerts: Vec<String>,
    current_alert_index: usize,
}

impl AccessVisualAlertSound {
    pub fn new() -> Self {
        AccessVisualAlertSound {
            alerts: Vec::new(),
            current_alert_index: 0,
        }
    }

    pub fn add_alert(&mut self, alert_message: String) {
        self.alerts.push(alert_message);
    }

    pub fn remove_alert(&mut self, index: usize) -> Option<String> {
        if index < self.alerts.len() {
            Some(self.alerts.remove(index))
        } else {
            None
        }
    }

    pub fn get_current_alert(&self) -> Option<&String> {
        if self.current_alert_index < self.alerts.len() {
            Some(&self.alerts[self.current_alert_index])
        } else {
            None
        }
    }

    pub fn next_alert(&mut self) -> Option<&String> {
        if self.current_alert_index < self.alerts.len() {
            let current = &self.alerts[self.current_alert_index];
            self.current_alert_index += 1;
            Some(current)
        } else {
            None
        }
    }

    pub fn reset_alerts(&mut self) {
        self.current_alert_index = 0;
    }
}
