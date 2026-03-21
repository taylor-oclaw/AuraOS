extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct AccessBounceKeys {
    keys: Vec<String>,
}

impl AccessBounceKeys {
    pub fn new() -> Self {
        AccessBounceKeys { keys: Vec::new() }
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
    fn test_access_bounce_keys() {
        let mut keys = AccessBounceKeys::new();
        assert_eq!(keys.list_keys(), Vec::<String>::new());

        keys.add_key(String::from("key1"));
        keys.add_key(String::from("key2"));
        assert_eq!(keys.has_key("key1"), true);
        assert_eq!(keys.has_key("key3"), false);

        let key_list = keys.list_keys();
        assert_eq!(key_list.len(), 2);
        assert!(key_list.contains(&String::from("key1")));
        assert!(key_list.contains(&String::from("key2")));

        assert_eq!(keys.remove_key("key1"), true);
        assert_eq!(keys.has_key("key1"), false);

        keys.clear_keys();
        assert_eq!(keys.list_keys(), Vec::<String>::new());
    }
}
