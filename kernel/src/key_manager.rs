extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct KeyManager {
    keys: Vec<String>,
}

impl KeyManager {
    pub fn new() -> Self {
        KeyManager { keys: Vec::new() }
    }

    pub fn add_key(&mut self, key: String) {
        if !self.keys.contains(&key) {
            self.keys.push(key);
        }
    }

    pub fn remove_key(&mut self, key: &str) {
        self.keys.retain(|k| k != key);
    }

    pub fn has_key(&self, key: &str) -> bool {
        self.keys.contains(&String::from(key))
    }

    pub fn list_keys(&self) -> Vec<String> {
        self.keys.clone()
    }

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }
}
