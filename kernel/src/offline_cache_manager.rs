extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct OfflineCacheManager {
    cache: Vec<(String, String)>,
}

impl OfflineCacheManager {
    pub fn new() -> Self {
        OfflineCacheManager {
            cache: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, key: &str, value: &str) {
        let key = String::from(key);
        let value = String::from(value);
        self.cache.push((key, value));
    }

    pub fn get_entry(&self, key: &str) -> Option<&String> {
        for (k, v) in &self.cache {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    pub fn remove_entry(&mut self, key: &str) {
        self.cache.retain(|(k, _)| k != key);
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    pub fn list_keys(&self) -> Vec<&String> {
        self.cache.iter().map(|(k, _)| k).collect()
    }
}
