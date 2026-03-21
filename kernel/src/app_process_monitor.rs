extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn app_process_monitor_init() {
    // Initialization logic for the process monitor module
}

#[no_mangle]
pub extern "C" fn app_process_monitor_exit() {
    // Cleanup logic for the process monitor module
}

pub struct ProcessMonitor {
    processes: Vec<String>,
}

impl ProcessMonitor {
    pub fn new() -> Self {
        ProcessMonitor {
            processes: Vec::new(),
        }
    }

    pub fn add_process(&mut self, process_name: &str) {
        self.processes.push(String::from(process_name));
    }

    pub fn remove_process(&mut self, process_name: &str) {
        if let Some(index) = self.processes.iter().position(|p| p == process_name) {
            self.processes.remove(index);
        }
    }

    pub fn get_processes(&self) -> &[String] {
        &self.processes
    }

    pub fn is_process_running(&self, process_name: &str) -> bool {
        self.processes.contains(&String::from(process_name))
    }

    pub fn count_processes(&self) -> usize {
        self.processes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_monitor() {
        let mut monitor = ProcessMonitor::new();
        assert_eq!(monitor.count_processes(), 0);

        monitor.add_process("process1");
        monitor.add_process("process2");
        assert_eq!(monitor.count_processes(), 2);
        assert!(monitor.is_process_running("process1"));
        assert!(!monitor.is_process_running("process3"));

        monitor.remove_process("process1");
        assert_eq!(monitor.count_processes(), 1);
        assert!(!monitor.is_process_running("process1"));

        let processes = monitor.get_processes();
        assert_eq!(processes.len(), 1);
        assert_eq!(processes[0], "process2");
    }
}
