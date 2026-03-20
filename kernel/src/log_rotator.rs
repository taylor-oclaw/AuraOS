extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct LogRotator {
    logs: Vec<String>,
    max_logs: usize,
}

impl LogRotator {
    pub fn new(max_logs: usize) -> Self {
        LogRotator {
            logs: Vec::new(),
            max_logs,
        }
    }

    pub fn add_log(&mut self, log: String) {
        if self.logs.len() >= self.max_logs {
            self.logs.remove(0);
        }
        self.logs.push(log);
    }

    pub fn get_logs(&self) -> &Vec<String> {
        &self.logs
    }

    pub fn clear_logs(&mut self) {
        self.logs.clear();
    }

    pub fn log_count(&self) -> usize {
        self.logs.len()
    }

    pub fn max_log_capacity(&self) -> usize {
        self.max_logs
    }
}
