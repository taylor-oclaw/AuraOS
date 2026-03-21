extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct EnterpriseAuditLog {
    entries: Vec<String>,
}

impl EnterpriseAuditLog {
    pub fn new() -> Self {
        EnterpriseAuditLog {
            entries: Vec::new(),
        }
    }

    pub fn log(&mut self, entry: &str) {
        self.entries.push(String::from(entry));
    }

    pub fn get_logs(&self) -> &[String] {
        &self.entries
    }

    pub fn clear_logs(&mut self) {
        self.entries.clear();
    }

    pub fn count_logs(&self) -> usize {
        self.entries.len()
    }

    pub fn find_log(&self, query: &str) -> Option<&String> {
        self.entries.iter().find(|entry| entry.contains(query))
    }
}
