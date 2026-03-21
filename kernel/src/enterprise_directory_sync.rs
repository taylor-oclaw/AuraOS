extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct EnterpriseDirectorySync {
    entries: Vec<String>,
}

impl EnterpriseDirectorySync {
    pub fn new() -> Self {
        EnterpriseDirectorySync {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: String) {
        if !self.entries.contains(&entry) {
            self.entries.push(entry);
        }
    }

    pub fn remove_entry(&mut self, entry: &str) {
        self.entries.retain(|e| e != entry);
    }

    pub fn contains_entry(&self, entry: &str) -> bool {
        self.entries.iter().any(|e| e == entry)
    }

    pub fn list_entries(&self) -> Vec<String> {
        self.entries.clone()
    }

    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }
}
