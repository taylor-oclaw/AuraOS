extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct AISecAuditDataSheet {
    entries: Vec<AuditEntry>,
}

impl AISecAuditDataSheet {
    pub fn new() -> Self {
        AISecAuditDataSheet {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: AuditEntry) {
        self.entries.push(entry);
    }

    pub fn get_entries(&self) -> &Vec<AuditEntry> {
        &self.entries
    }

    pub fn remove_entry_by_id(&mut self, id: u32) -> Option<AuditEntry> {
        let pos = self.entries.iter().position(|e| e.id == id);
        match pos {
            Some(index) => Some(self.entries.remove(index)),
            None => None,
        }
    }

    pub fn find_entry_by_id(&self, id: u32) -> Option<&AuditEntry> {
        self.entries.iter().find(|e| e.id == id)
    }
}

pub struct AuditEntry {
    id: u32,
    description: String,
    timestamp: u64,
    severity: Severity,
}

impl AuditEntry {
    pub fn new(id: u32, description: String, timestamp: u64, severity: Severity) -> Self {
        AuditEntry {
            id,
            description,
            timestamp,
            severity,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn get_severity(&self) -> Severity {
        self.severity
    }
}

#[derive(Copy, Clone)]
pub enum Severity {
    Low,
    Medium,
    High,
}
