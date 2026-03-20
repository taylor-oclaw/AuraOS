extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentCheckpoint {
    name: String,
    version: u32,
    status: String,
    logs: Vec<String>,
    tasks_completed: usize,
}

impl AgentCheckpoint {
    pub fn new(name: &str, version: u32) -> Self {
        AgentCheckpoint {
            name: String::from(name),
            version,
            status: String::from("Active"),
            logs: Vec::new(),
            tasks_completed: 0,
        }
    }

    pub fn update_status(&mut self, new_status: &str) {
        self.status = String::from(new_status);
    }

    pub fn log_event(&mut self, event: &str) {
        self.logs.push(String::from(event));
    }

    pub fn increment_tasks_completed(&mut self) {
        self.tasks_completed += 1;
    }

    pub fn get_logs(&self) -> &[String] {
        &self.logs
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }
}
