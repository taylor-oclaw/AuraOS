extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct AccessFilterKeys {
    keys: Vec<String>,
}

impl AccessFilterKeys {
    pub fn new() -> Self {
        AccessFilterKeys { keys: Vec::new() }
    }

    pub fn add_key(&mut self, key: String) {
        if !self.keys.contains(&key) {
            self.keys.push(key);
        }
    }

    pub fn remove_key(&mut self, key: &str) -> bool {
        let pos = self.keys.iter().position(|k| k == key);
        if let Some(index) = pos {
            self.keys.remove(index);
            true
        } else {
            false
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_filter_keys() {
        let mut filter = AccessFilterKeys::new();
        assert_eq!(filter.list_keys(), Vec::<String>::new());

        filter.add_key(String::from("key1"));
        assert!(filter.has_key("key1"));
        assert_eq!(filter.list_keys(), vec![String::from("key1")]);

        filter.add_key(String::from("key2"));
        assert!(filter.has_key("key2"));
        assert_eq!(filter.list_keys().len(), 2);

        assert!(filter.remove_key("key1"));
        assert!(!filter.has_key("key1"));
        assert_eq!(filter.list_keys(), vec![String::from("key2")]);

        filter.clear_keys();
        assert_eq!(filter.list_keys(), Vec::<String>::new());
    }
}
