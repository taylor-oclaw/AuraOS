extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AISafetyRegistry {
    entries: Vec<(String, String)>,
}

impl AISafetyRegistry {
    pub fn new() -> Self {
        AISafetyRegistry {
            entries: Vec::new(),
        }
    }

    pub fn register(&mut self, key: &str, value: &str) {
        let key = String::from(key);
        let value = String::from(value);
        self.entries.push((key, value));
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        for (k, v) in &self.entries {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    pub fn remove(&mut self, key: &str) {
        self.entries.retain(|(k, _)| k != key);
    }

    pub fn contains_key(&self, key: &str) -> bool {
        for (k, _) in &self.entries {
            if k == key {
                return true;
            }
        }
        false
    }

    pub fn list_keys(&self) -> Vec<&String> {
        self.entries.iter().map(|(k, _)| k).collect()
    }
}
