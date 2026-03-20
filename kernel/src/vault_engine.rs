extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct VaultEngine {
    secrets: Vec<(String, String)>,
}

impl VaultEngine {
    pub fn new() -> Self {
        VaultEngine {
            secrets: Vec::new(),
        }
    }

    pub fn add_secret(&mut self, key: &str, value: &str) {
        let secret_key = String::from(key);
        let secret_value = String::from(value);
        self.secrets.push((secret_key, secret_value));
    }

    pub fn get_secret(&self, key: &str) -> Option<&String> {
        for (k, v) in &self.secrets {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    pub fn remove_secret(&mut self, key: &str) {
        self.secrets.retain(|(k, _)| k != key);
    }

    pub fn list_secrets(&self) -> Vec<&String> {
        self.secrets.iter().map(|(_, v)| v).collect()
    }

    pub fn secret_count(&self) -> usize {
        self.secrets.len()
    }
}
