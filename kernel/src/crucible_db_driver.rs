extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn crucible_db_driver_init() {
}

pub extern "C" fn crucible_db_driver_exit() {
}

pub struct CrucibleDB {
    data: Vec<(String, String)>,
}

impl CrucibleDB {
    pub fn new() -> Self {
        CrucibleDB { data: Vec::new() }
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        let pair = (String::from(key), String::from(value));
        self.data.push(pair);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        for (k, v) in &self.data {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    pub fn remove(&mut self, key: &str) {
        self.data.retain(|(k, _)| k != key);
    }

    pub fn contains_key(&self, key: &str) -> bool {
        for (k, _) in &self.data {
            if k == key {
                return true;
            }
        }
        false
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crucible_db() {
        let mut db = CrucibleDB::new();
        assert_eq!(db.len(), 0);

        db.insert("key1", "value1");
        assert_eq!(db.len(), 1);
        assert_eq!(db.get("key1"), Some(&String::from("value1")));

        db.remove("key1");
        assert_eq!(db.len(), 0);
        assert_eq!(db.get("key1"), None);

        db.insert("key2", "value2");
        assert_eq!(db.contains_key("key2"), true);
    }
}
