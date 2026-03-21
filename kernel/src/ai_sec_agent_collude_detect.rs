extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct AiSecAgentColludeDetect {
    // Example fields for the struct
    processes: Vec<String>,
    alerts: Vec<String>,
}

impl AiSecAgentColludeDetect {
    pub fn new() -> Self {
        AiSecAgentColludeDetect {
            processes: Vec::new(),
            alerts: Vec::new(),
        }
    }

    pub fn add_process(&mut self, process_name: &str) {
        self.processes.push(process_name.to_string());
    }

    pub fn remove_process(&mut self, process_name: &str) -> bool {
        if let Some(index) = self.processes.iter().position(|p| p == process_name) {
            self.processes.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_processes(&self) -> Vec<String> {
        self.processes.clone()
    }

    pub fn detect_collusion(&mut self, process_name: &str) -> bool {
        // Example logic for collusion detection
        if self.processes.contains(&process_name.to_string()) {
            self.alerts.push(format!("Collusion detected with {}", process_name));
            true
        } else {
            false
        }
    }

    pub fn get_alerts(&self) -> Vec<String> {
        self.alerts.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_process() {
        let mut agent = AiSecAgentColludeDetect::new();
        agent.add_process("process1");
        assert_eq!(agent.get_processes(), vec![String::from("process1")]);
    }

    #[test]
    fn test_remove_process() {
        let mut agent = AiSecAgentColludeDetect::new();
        agent.add_process("process1");
        assert!(agent.remove_process("process1"));
        assert_eq!(agent.get_processes(), Vec::<String>::new());
    }

    #[test]
    fn test_detect_collusion() {
        let mut agent = AiSecAgentColludeDetect::new();
        agent.add_process("process1");
        assert!(agent.detect_collusion("process1"));
        assert_eq!(agent.get_alerts(), vec![String::from("Collusion detected with process1")]);
    }
}
