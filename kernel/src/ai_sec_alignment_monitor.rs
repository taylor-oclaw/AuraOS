extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct AiSecAlignmentMonitor {
    processes: Vec<String>,
    alerts: Vec<String>,
}

impl AiSecAlignmentMonitor {
    pub fn new() -> Self {
        AiSecAlignmentMonitor {
            processes: Vec::new(),
            alerts: Vec::new(),
        }
    }

    pub fn add_process(&mut self, process_name: &str) {
        self.processes.push(String::from(process_name));
    }

    pub fn remove_process(&mut self, process_name: &str) -> bool {
        if let Some(index) = self.processes.iter().position(|p| p == process_name) {
            self.processes.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_processes(&self) -> Vec<String> {
        self.processes.clone()
    }

    pub fn generate_alert(&mut self, alert_message: &str) {
        self.alerts.push(String::from(alert_message));
    }

    pub fn get_alerts(&self) -> Vec<String> {
        self.alerts.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_remove_process() {
        let mut monitor = AiSecAlignmentMonitor::new();
        monitor.add_process("process1");
        assert_eq!(monitor.list_processes(), vec![String::from("process1")]);
        assert!(monitor.remove_process("process1"));
        assert_eq!(monitor.list_processes(), Vec::<String>::new());
    }

    #[test]
    fn test_generate_and_get_alerts() {
        let mut monitor = AiSecAlignmentMonitor::new();
        monitor.generate_alert("Alert 1");
        monitor.generate_alert("Alert 2");
        assert_eq!(
            monitor.get_alerts(),
            vec![String::from("Alert 1"), String::from("Alert 2")]
        );
    }
}
