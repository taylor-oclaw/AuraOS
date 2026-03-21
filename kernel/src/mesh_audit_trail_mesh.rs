extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshAuditTrail {
    entries: Vec<String>,
}

impl MeshAuditTrail {
    pub fn new() -> Self {
        MeshAuditTrail {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: String) {
        self.entries.push(entry);
    }

    pub fn get_entries(&self) -> &Vec<String> {
        &self.entries
    }

    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    pub fn has_entry(&self, entry: &str) -> bool {
        self.entries.iter().any(|e| e == entry)
    }

    pub fn count_entries(&self) -> usize {
        self.entries.len()
    }
}
