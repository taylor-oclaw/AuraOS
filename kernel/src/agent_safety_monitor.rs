extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn agent_safety_monitor_init() {
    // Initialization logic for the module
}

pub extern "C" fn agent_safety_monitor_exit() {
    // Cleanup logic for the module
}

pub struct AgentSafetyMonitor {
    alerts: Vec<String>,
    violations: u32,
}

impl AgentSafetyMonitor {
    pub fn new() -> Self {
        AgentSafetyMonitor {
            alerts: Vec::new(),
            violations: 0,
        }
    }

    pub fn add_alert(&mut self, alert_message: &str) {
        let message = String::from(alert_message);
        self.alerts.push(message);
    }

    pub fn get_alerts(&self) -> &[String] {
        &self.alerts
    }

    pub fn record_violation(&mut self) {
        self.violations += 1;
    }

    pub fn get_violation_count(&self) -> u32 {
        self.violations
    }

    pub fn clear_alerts(&mut self) {
        self.alerts.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_safety_monitor() {
        let mut monitor = AgentSafetyMonitor::new();

        assert_eq!(monitor.get_violation_count(), 0);
        assert!(monitor.get_alerts().is_empty());

        monitor.add_alert("Unauthorized access attempt");
        monitor.record_violation();

        assert_eq!(monitor.get_violation_count(), 1);
        assert_eq!(monitor.get_alerts().len(), 1);
        assert_eq!(monitor.get_alerts()[0], "Unauthorized access attempt");

        monitor.clear_alerts();
        assert!(monitor.get_alerts().is_empty());
    }
}
