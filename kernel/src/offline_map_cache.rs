extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct OfflineMapCache {
    cache: Vec<(String, String)>,
}

impl OfflineMapCache {
    pub fn new() -> Self {
        OfflineMapCache { cache: Vec::new() }
    }

    pub fn add(&mut self, key: &str, value: &str) {
        let key = String::from(key);
        let value = String::from(value);
        self.cache.push((key, value));
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        for (k, v) in &self.cache {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    pub fn remove(&mut self, key: &str) -> bool {
        let pos = self.cache.iter().position(|(k, _)| k == key);
        if let Some(index) = pos {
            self.cache.remove(index);
            true
        } else {
            false
        }
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.cache.iter().any(|(k, _)| k == key)
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }
}
