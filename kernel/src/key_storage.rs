extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct KeyStorage {
    keys: Vec<String>,
}

impl KeyStorage {
    pub fn new() -> Self {
        KeyStorage { keys: Vec::new() }
    }

    pub fn add_key(&mut self, key: String) {
        if !self.keys.contains(&key) {
            self.keys.push(key);
        }
    }

    pub fn remove_key(&mut self, key: &str) {
        self.keys.retain(|k| k != key);
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.keys.iter().any(|k| k == key)
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.keys.clone()
    }

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }
}
