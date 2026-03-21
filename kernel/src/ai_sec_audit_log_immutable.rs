extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct AISecAuditLogImmutable {
    logs: Vec<String>,
}

impl AISecAuditLogImmutable {
    pub fn new() -> Self {
        AISecAuditLogImmutable { logs: Vec::new() }
    }

    pub fn add_log(&mut self, log_entry: &str) {
        self.logs.push(String::from(log_entry));
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

    pub fn find_log(&self, search_term: &str) -> Option<&String> {
        self.logs.iter().find(|log| log.contains(search_term))
    }
}
