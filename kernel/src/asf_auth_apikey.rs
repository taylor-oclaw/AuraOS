extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct ApiKeyAuth {
    api_keys: Vec<String>,
}

impl ApiKeyAuth {
    pub fn new() -> Self {
        ApiKeyAuth {
            api_keys: Vec::new(),
        }
    }

    pub fn add_api_key(&mut self, key: &str) {
        if !self.api_keys.contains(&key.to_string()) {
            self.api_keys.push(key.to_string());
        }
    }

    pub fn remove_api_key(&mut self, key: &str) {
        self.api_keys.retain(|k| k != key);
    }

    pub fn is_valid_api_key(&self, key: &str) -> bool {
        self.api_keys.contains(&key.to_string())
    }

    pub fn list_api_keys(&self) -> Vec<String> {
        self.api_keys.clone()
    }

    pub fn clear_api_keys(&mut self) {
        self.api_keys.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_key_auth() {
        let mut auth = ApiKeyAuth::new();

        assert!(!auth.is_valid_api_key("key1"));
        auth.add_api_key("key1");
        assert!(auth.is_valid_api_key("key1"));

        auth.remove_api_key("key1");
        assert!(!auth.is_valid_api_key("key1"));

        auth.add_api_key("key2");
        let keys = auth.list_api_keys();
        assert_eq!(keys, vec![String::from("key2")]);

        auth.clear_api_keys();
        assert!(auth.list_api_keys().is_empty());
    }
}
