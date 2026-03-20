extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AILogger {
    logs: Vec<String>,
}

impl AILogger {
    pub fn new() -> Self {
        AILogger { logs: Vec::new() }
    }

    pub fn log(&mut self, message: &str) {
        self.logs.push(String::from(message));
    }

    pub fn get_logs(&self) -> &[String] {
        &self.logs
    }

    pub fn clear_logs(&mut self) {
        self.logs.clear();
    }

    pub fn log_count(&self) -> usize {
        self.logs.len()
    }

    pub fn find_log(&self, message: &str) -> Option<&String> {
        self.logs.iter().find(|&&log| log == message)
    }
}
