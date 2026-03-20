extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod secret_manager {
    use super::*;

    pub struct SecretManager {
        secrets: Vec<(String, String)>,
    }

    impl SecretManager {
        pub fn new() -> Self {
            SecretManager {
                secrets: Vec::new(),
            }
        }

        pub fn add_secret(&mut self, key: &str, value: &str) {
            let key = String::from(key);
            let value = String::from(value);
            self.secrets.push((key, value));
        }

        pub fn get_secret(&self, key: &str) -> Option<&String> {
            for (k, v) in &self.secrets {
                if k == key {
                    return Some(v);
                }
            }
            None
        }

        pub fn remove_secret(&mut self, key: &str) {
            self.secrets.retain(|(k, _)| k != key);
        }

        pub fn list_secrets(&self) -> Vec<&String> {
            self.secrets.iter().map(|(_, v)| v).collect()
        }

        pub fn count_secrets(&self) -> usize {
            self.secrets.len()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::secret_manager::*;

    #[test]
    fn test_secret_manager() {
        let mut manager = SecretManager::new();
        assert_eq!(manager.count_secrets(), 0);

        manager.add_secret("password", "123456");
        assert_eq!(manager.count_secrets(), 1);
        assert_eq!(manager.get_secret("password"), Some(&String::from("123456")));

        manager.add_secret("username", "admin");
        assert_eq!(manager.count_secrets(), 2);
        assert_eq!(manager.get_secret("username"), Some(&String::from("admin")));

        let secrets = manager.list_secrets();
        assert_eq!(secrets.len(), 2);
        assert!(secrets.contains(&&String::from("123456")));
        assert!(secrets.contains(&&String::from("admin")));

        manager.remove_secret("password");
        assert_eq!(manager.count_secrets(), 1);
        assert_eq!(manager.get_secret("password"), None);

        manager.remove_secret("username");
        assert_eq!(manager.count_secrets(), 0);
        assert_eq!(manager.get_secret("username"), None);
    }
}
