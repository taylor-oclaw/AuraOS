extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    loop {}
}

struct CacheEntry {
    key: String,
    value: Vec<u8>,
}

impl CacheEntry {
    fn new(key: String, value: Vec<u8>) -> Self {
        CacheEntry { key, value }
    }
}

pub struct AICache {
    entries: Vec<CacheEntry>,
}

impl AICache {
    pub fn new() -> Self {
        AICache { entries: Vec::new() }
    }

    pub fn insert(&mut self, key: String, value: Vec<u8>) {
        let entry = CacheEntry::new(key, value);
        self.entries.push(entry);
    }

    pub fn get(&self, key: &str) -> Option<&Vec<u8>> {
        for entry in &self.entries {
            if entry.key == key {
                return Some(&entry.value);
            }
        }
        None
    }

    pub fn remove(&mut self, key: &str) {
        self.entries.retain(|e| e.key != key);
    }

    pub fn contains_key(&self, key: &str) -> bool {
        for entry in &self.entries {
            if entry.key == key {
                return true;
            }
        }
        false
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
