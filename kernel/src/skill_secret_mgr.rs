extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct SkillSecretMgr {
    secrets: Vec<(String, String)>,
}

impl SkillSecretMgr {
    pub fn new() -> Self {
        SkillSecretMgr {
            secrets: Vec::new(),
        }
    }

    pub fn add_secret(&mut self, key: &str, value: &str) {
        let key_str = String::from(key);
        let value_str = String::from(value);
        self.secrets.push((key_str, value_str));
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

    pub fn secret_count(&self) -> usize {
        self.secrets.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_secret_mgr() {
        let mut mgr = SkillSecretMgr::new();
        assert_eq!(mgr.secret_count(), 0);

        mgr.add_secret("key1", "value1");
        assert_eq!(mgr.secret_count(), 1);
        assert_eq!(mgr.get_secret("key1").unwrap(), "value1");

        mgr.add_secret("key2", "value2");
        assert_eq!(mgr.secret_count(), 2);
        assert_eq!(mgr.get_secret("key2").unwrap(), "value2");

        let secrets = mgr.list_secrets();
        assert_eq!(secrets.len(), 2);
        assert!(secrets.contains(&"value1"));
        assert!(secrets.contains(&"value2"));

        mgr.remove_secret("key1");
        assert_eq!(mgr.secret_count(), 1);
        assert!(mgr.get_secret("key1").is_none());

        mgr.remove_secret("key2");
        assert_eq!(mgr.secret_count(), 0);
    }
}
