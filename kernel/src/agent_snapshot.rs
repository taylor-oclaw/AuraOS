extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut snapshot = AgentSnapshot::new();
    snapshot.log_status();
    snapshot.add_process(String::from("init"));
    snapshot.add_process(String::from("sshd"));
    snapshot.remove_process(String::from("init"));
    snapshot.list_processes();
    snapshot.clear_processes();
}

pub struct AgentSnapshot {
    processes: Vec<String>,
}

impl AgentSnapshot {
    pub fn new() -> Self {
        AgentSnapshot {
            processes: Vec::new(),
        }
    }

    pub fn add_process(&mut self, process_name: String) {
        if !self.processes.contains(&process_name) {
            self.processes.push(process_name);
        }
    }

    pub fn remove_process(&mut self, process_name: String) {
        self.processes.retain(|p| *p != process_name);
    }

    pub fn list_processes(&self) -> Vec<String> {
        self.processes.clone()
    }

    pub fn clear_processes(&mut self) {
        self.processes.clear();
    }

    pub fn log_status(&self) {
        // Simulate logging status
        for process in &self.processes {
            // Placeholder for logging logic
        }
    }
}
