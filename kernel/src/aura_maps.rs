extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod aura_maps {
    use super::*;

    pub struct AuraMap {
        entries: Vec<(String, String)>,
    }

    impl AuraMap {
        pub fn new() -> Self {
            AuraMap {
                entries: Vec::new(),
            }
        }

        pub fn insert(&mut self, key: &str, value: &str) {
            let entry = (String::from(key), String::from(value));
            self.entries.push(entry);
        }

        pub fn get(&self, key: &str) -> Option<&String> {
            for (k, v) in &self.entries {
                if k == key {
                    return Some(v);
                }
            }
            None
        }

        pub fn remove(&mut self, key: &str) {
            self.entries.retain(|(k, _)| k != key);
        }

        pub fn contains_key(&self, key: &str) -> bool {
            for (k, _) in &self.entries {
                if k == key {
                    return true;
                }
            }
            false
        }

        pub fn len(&self) -> usize {
            self.entries.len()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::aura_maps::AuraMap;

    #[test]
    fn test_aura_map() {
        let mut map = AuraMap::new();
        assert_eq!(map.len(), 0);

        map.insert("key1", "value1");
        assert_eq!(map.len(), 1);
        assert_eq!(map.get("key1"), Some(&String::from("value1")));

        map.insert("key2", "value2");
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("key2"), Some(&String::from("value2")));

        assert!(map.contains_key("key1"));
        assert!(!map.contains_key("key3"));

        map.remove("key1");
        assert_eq!(map.len(), 1);
        assert_eq!(map.get("key1"), None);

        map.remove("key2");
        assert_eq!(map.len(), 0);
    }
}
