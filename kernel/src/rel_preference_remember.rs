extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct PreferenceRemember {
    preferences: Vec<(String, String)>,
}

impl PreferenceRemember {
    pub fn new() -> Self {
        PreferenceRemember {
            preferences: Vec::new(),
        }
    }

    pub fn add_preference(&mut self, key: &str, value: &str) {
        let key = String::from(key);
        let value = String::from(value);
        self.preferences.push((key, value));
    }

    pub fn get_preference(&self, key: &str) -> Option<&String> {
        for (k, v) in &self.preferences {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    pub fn remove_preference(&mut self, key: &str) {
        self.preferences.retain(|(k, _)| k != key);
    }

    pub fn list_preferences(&self) -> Vec<&String> {
        self.preferences.iter().map(|(_, v)| v).collect()
    }

    pub fn clear_preferences(&mut self) {
        self.preferences.clear();
    }
}
