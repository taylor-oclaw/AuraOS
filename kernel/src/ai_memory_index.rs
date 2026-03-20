extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct AIMemoryIndex {
    index: Vec<(String, usize)>,
}

impl AIMemoryIndex {
    pub fn new() -> Self {
        AIMemoryIndex { index: Vec::new() }
    }

    pub fn add_entry(&mut self, key: String, value: usize) {
        self.index.push((key, value));
    }

    pub fn get_value(&self, key: &str) -> Option<usize> {
        for (k, v) in &self.index {
            if k == key {
                return Some(*v);
            }
        }
        None
    }

    pub fn remove_entry(&mut self, key: &str) {
        self.index.retain(|(k, _)| k != key);
    }

    pub fn contains_key(&self, key: &str) -> bool {
        for (k, _) in &self.index {
            if k == key {
                return true;
            }
        }
        false
    }

    pub fn get_all_keys(&self) -> Vec<String> {
        self.index.iter().map(|(k, _)| k.clone()).collect()
    }
}
