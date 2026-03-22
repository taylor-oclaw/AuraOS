extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct OpenApiAuthHandler {
    api_keys: Vec<String>,
}

impl OpenApiAuthHandler {
    pub fn new() -> Self {
        OpenApiAuthHandler {
            api_keys: Vec::new(),
        }
    }

    pub fn add_api_key(&mut self, key: &str) {
        self.api_keys.push(key.to_string());
    }

    pub fn remove_api_key(&mut self, key: &str) -> bool {
        if let Some(index) = self.api_keys.iter().position(|k| k == key) {
            self.api_keys.remove(index);
            true
        } else {
            false
        }
    }

    pub fn is_valid_api_key(&self, key: &str) -> bool {
        self.api_keys.contains(&key.to_string())
    }

    pub fn list_api_keys(&self) -> Vec<String> {
        self.api_keys.clone()
    }

    pub fn clear_api_keys(&mut self) {
        self.api_keys.clear();
    }
}