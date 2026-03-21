extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct WineRegistry {
    entries: Vec<(String, String)>,
}

impl WineRegistry {
    pub fn new() -> Self {
        WineRegistry {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, key: &str, value: &str) {
        let key_str = String::from(key);
        let value_str = String::from(value);
        self.entries.push((key_str, value_str));
    }

    pub fn get_value(&self, key: &str) -> Option<&String> {
        for (k, v) in &self.entries {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    pub fn remove_entry(&mut self, key: &str) {
        self.entries.retain(|(k, _)| k != key);
    }

    pub fn list_entries(&self) -> Vec<&String> {
        self.entries.iter().map(|(_, v)| v).collect()
    }

    pub fn count_entries(&self) -> usize {
        self.entries.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wine_registry() {
        let mut registry = WineRegistry::new();
        assert_eq!(registry.count_entries(), 0);

        registry.add_entry("key1", "value1");
        registry.add_entry("key2", "value2");
        assert_eq!(registry.count_entries(), 2);

        assert_eq!(registry.get_value("key1"), Some(&String::from("value1")));
        assert_eq!(registry.get_value("key3"), None);

        registry.remove_entry("key1");
        assert_eq!(registry.count_entries(), 1);
        assert_eq!(registry.get_value("key1"), None);

        let entries = registry.list_entries();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0], &String::from("value2"));
    }
}
