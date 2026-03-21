extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct FamilyHubSharedStorage {
    data: Vec<(String, String)>,
}

impl FamilyHubSharedStorage {
    pub fn new() -> Self {
        FamilyHubSharedStorage { data: Vec::new() }
    }

    pub fn add_entry(&mut self, key: &str, value: &str) {
        let key_str = String::from(key);
        let value_str = String::from(value);
        self.data.push((key_str, value_str));
    }

    pub fn get_value(&self, key: &str) -> Option<&String> {
        for (k, v) in &self.data {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    pub fn remove_entry(&mut self, key: &str) {
        self.data.retain(|(k, _)| k != key);
    }

    pub fn list_keys(&self) -> Vec<String> {
        let mut keys = Vec::new();
        for (key, _) in &self.data {
            keys.push(key.clone());
        }
        keys
    }

    pub fn clear_storage(&mut self) {
        self.data.clear();
    }
}
