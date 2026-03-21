extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct AuditTrail {
    entries: Vec<String>,
}

impl AuditTrail {
    pub fn new() -> Self {
        AuditTrail {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: &str) {
        self.entries.push(String::from(entry));
    }

    pub fn get_entries(&self) -> &[String] {
        &self.entries
    }

    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    pub fn count_entries(&self) -> usize {
        self.entries.len()
    }

    pub fn find_entry(&self, entry: &str) -> Option<&String> {
        self.entries.iter().find(|e| e == entry)
    }
}
