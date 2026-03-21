extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn init_module() -> i32 {
    0
}

pub extern "C" fn cleanup_module() -> i32 {
    0
}

pub struct ActivityLog {
    entries: Vec<String>,
}

impl ActivityLog {
    pub fn new() -> Self {
        ActivityLog {
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

    pub fn count_entries(&self) -> usize {
        self.entries.len()
    }

    pub fn find_entry(&self, query: &str) -> Option<&String> {
        self.entries.iter().find(|entry| entry.contains(query))
    }
}
