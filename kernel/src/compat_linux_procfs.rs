extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CompatLinuxProcfs {
    entries: Vec<String>,
}

impl CompatLinuxProcfs {
    pub fn new() -> Self {
        CompatLinuxProcfs {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: String) {
        self.entries.push(entry);
    }

    pub fn remove_entry(&mut self, index: usize) -> Option<String> {
        if index < self.entries.len() {
            Some(self.entries.remove(index))
        } else {
            None
        }
    }

    pub fn get_entry(&self, index: usize) -> Option<&String> {
        self.entries.get(index)
    }

    pub fn list_entries(&self) -> Vec<&String> {
        self.entries.iter().collect()
    }

    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }
}
