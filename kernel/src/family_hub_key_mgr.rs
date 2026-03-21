extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod family_hub_key_mgr {
    use super::*;

    pub struct KeyManager {
        keys: Vec<String>,
    }

    impl KeyManager {
        pub fn new() -> Self {
            KeyManager { keys: Vec::new() }
        }

        pub fn add_key(&mut self, key: String) {
            if !self.keys.contains(&key) {
                self.keys.push(key);
            }
        }

        pub fn remove_key(&mut self, key: &str) {
            self.keys.retain(|k| k != key);
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_manager() {
        let mut km = family_hub_key_mgr::KeyManager::new();

        assert_eq!(km.list_keys(), Vec::<String>::new());

        km.add_key(String::from("key1"));
        assert_eq!(km.has_key("key1"), true);
        assert_eq!(km.list_keys(), vec![String::from("key1")]);

        km.add_key(String::from("key2"));
        assert_eq!(km.has_key("key2"), true);
        assert_eq!(km.list_keys().len(), 2);

        km.remove_key("key1");
        assert_eq!(km.has_key("key1"), false);
        assert_eq!(km.list_keys(), vec![String::from("key2")]);

        km.clear_keys();
        assert_eq!(km.has_key("key2"), false);
        assert_eq!(km.list_keys(), Vec::<String>::new());
    }
}
