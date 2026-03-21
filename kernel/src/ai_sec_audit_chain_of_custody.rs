extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct AiSecAuditChainOfCustody {
    entries: Vec<String>,
}

impl AiSecAuditChainOfCustody {
    pub fn new() -> Self {
        AiSecAuditChainOfCustody {
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

    pub fn has_entry(&self, entry: &str) -> bool {
        self.entries.iter().any(|e| e == entry)
    }

    pub fn count_entries(&self) -> usize {
        self.entries.len()
    }
}
