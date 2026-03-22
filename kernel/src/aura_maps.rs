extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialize the module if needed
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Clean up the module if needed
}

struct AuraMaps {
    map: Vec<(String, String)>,
}

impl AuraMaps {
    pub fn new() -> Self {
        AuraMaps { map: Vec::new() }
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        let key_str = String::from(key);
        let value_str = String::from(value);
        self.map.push((key_str, value_str));
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        for (k, v) in &self.map {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    pub fn remove(&mut self, key: &str) {
        self.map.retain(|(k, _)| k != key);
    }

    pub fn contains_key(&self, key: &str) -> bool {
        for (k, _) in &self.map {
            if k == key {
                return true;
            }
        }
        false
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aura_maps() {
        let mut aura_maps = AuraMaps::new();
        assert_eq!(aura_maps.len(), 0);

        aura_maps.insert("key1", "value1");
        assert_eq!(aura_maps.len(), 1);
        assert_eq!(aura_maps.get("key1"), Some(&String::from("value1")));

        aura_maps.insert("key2", "value2");
        assert_eq!(aura_maps.len(), 2);
        assert_eq!(aura_maps.get("key2"), Some(&String::from("value2")));

        aura_maps.remove("key1");
        assert_eq!(aura_maps.len(), 1);
        assert_eq!(aura_maps.get("key1"), None);

        assert!(aura_maps.contains_key("key2"));
        assert!(!aura_maps.contains_key("key3"));
    }
}