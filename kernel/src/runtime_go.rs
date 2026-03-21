extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct RuntimeGo {
    processes: Vec<String>,
}

impl RuntimeGo {
    pub fn new() -> Self {
        RuntimeGo {
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

    pub fn list_processes(&self) -> Vec<String> {
        self.processes.clone()
    }

    pub fn has_process(&self, process_name: &str) -> bool {
        self.processes.contains(&String::from(process_name))
    }

    pub fn count_processes(&self) -> usize {
        self.processes.len()
    }
}
