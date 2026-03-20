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

pub struct AuraSearchSpotlight {
    index: Vec<String>,
    data: Vec<String>,
}

impl AuraSearchSpotlight {
    pub fn new() -> Self {
        AuraSearchSpotlight {
            index: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, key: &str, value: &str) {
        self.index.push(key.to_string());
        self.data.push(value.to_string());
    }

    pub fn search(&self, query: &str) -> Option<&String> {
        for (i, index_key) in self.index.iter().enumerate() {
            if index_key == query {
                return Some(&self.data[i]);
            }
        }
        None
    }

    pub fn remove_entry(&mut self, key: &str) {
        if let Some(index) = self.index.iter().position(|x| x == key) {
            self.index.remove(index);
            self.data.remove(index);
        }
    }

    pub fn list_entries(&self) -> Vec<&String> {
        self.index.iter().collect()
    }

    pub fn count_entries(&self) -> usize {
        self.index.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_entry() {
        let mut spotlight = AuraSearchSpotlight::new();
        spotlight.add_entry("key1", "value1");
        assert_eq!(spotlight.count_entries(), 1);
    }

    #[test]
    fn test_search() {
        let mut spotlight = AuraSearchSpotlight::new();
        spotlight.add_entry("key1", "value1");
        assert_eq!(spotlight.search("key1"), Some(&String::from("value1")));
    }

    #[test]
    fn test_remove_entry() {
        let mut spotlight = AuraSearchSpotlight::new();
        spotlight.add_entry("key1", "value1");
        spotlight.remove_entry("key1");
        assert_eq!(spotlight.count_entries(), 0);
    }

    #[test]
    fn test_list_entries() {
        let mut spotlight = AuraSearchSpotlight::new();
        spotlight.add_entry("key1", "value1");
        spotlight.add_entry("key2", "value2");
        let entries = spotlight.list_entries();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0], &String::from("key1"));
        assert_eq!(entries[1], &String::from("key2"));
    }

    #[test]
    fn test_count_entries() {
        let mut spotlight = AuraSearchSpotlight::new();
        spotlight.add_entry("key1", "value1");
        spotlight.add_entry("key2", "value2");
        assert_eq!(spotlight.count_entries(), 2);
    }
}
