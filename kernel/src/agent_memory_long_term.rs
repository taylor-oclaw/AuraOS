extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct AgentMemoryLongTerm {
    data: Vec<String>,
}

impl AgentMemoryLongTerm {
    pub fn new() -> Self {
        AgentMemoryLongTerm {
            data: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: String) {
        self.data.push(entry);
    }

    pub fn get_entry(&self, index: usize) -> Option<&String> {
        self.data.get(index)
    }

    pub fn remove_entry(&mut self, index: usize) -> Option<String> {
        if index < self.data.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }

    pub fn get_all_entries(&self) -> &[String] {
        &self.data
    }

    pub fn clear_entries(&mut self) {
        self.data.clear();
    }
}
