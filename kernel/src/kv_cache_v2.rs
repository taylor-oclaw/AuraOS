extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct KVCachev2 {
    cache: Vec<(String, String)>,
}

impl KVCachev2 {
    pub fn new() -> Self {
        KVCachev2 { cache: Vec::new() }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        for entry in self.cache.iter_mut() {
            if entry.0 == key {
                entry.1 = String::from(value);
                return;
            }
        }
        self.cache.push((String::from(key), String::from(value)));
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        for (k, v) in self.cache.iter() {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    pub fn delete(&mut self, key: &str) {
        self.cache.retain(|(k, _)| k != key);
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }

    pub fn size(&self) -> usize {
        self.cache.len()
    }
}
