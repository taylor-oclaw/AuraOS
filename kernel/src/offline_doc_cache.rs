extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct OfflineDocCache {
    cache: Vec<(String, String)>,
}

impl OfflineDocCache {
    pub fn new() -> Self {
        OfflineDocCache {
            cache: Vec::new(),
        }
    }

    pub fn add_document(&mut self, key: &str, content: &str) {
        let key = String::from(key);
        let content = String::from(content);
        self.cache.push((key, content));
    }

    pub fn get_document(&self, key: &str) -> Option<&String> {
        for (k, v) in &self.cache {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    pub fn remove_document(&mut self, key: &str) {
        self.cache.retain(|(k, _)| k != key);
    }

    pub fn contains_key(&self, key: &str) -> bool {
        for (k, _) in &self.cache {
            if k == key {
                return true;
            }
        }
        false
    }

    pub fn list_documents(&self) -> Vec<&String> {
        self.cache.iter().map(|(k, _)| k).collect()
    }
}
