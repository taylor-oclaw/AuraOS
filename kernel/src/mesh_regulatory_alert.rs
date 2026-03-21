extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshRegulatoryAlert {
    alerts: Vec<String>,
}

impl MeshRegulatoryAlert {
    pub fn new() -> Self {
        MeshRegulatoryAlert {
            alerts: Vec::new(),
        }
    }

    pub fn add_alert(&mut self, alert: String) {
        self.alerts.push(alert);
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

    pub fn has_alerts(&self) -> bool {
        !self.alerts.is_empty()
    }

    pub fn clear_alerts(&mut self) {
        self.alerts.clear();
    }
}
