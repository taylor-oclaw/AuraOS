extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut memory_forget = AIMemoryForget::new();
    memory_forget.store_data("example_key", "example_value");
    let value = memory_forget.retrieve_data("example_key").unwrap();
    assert_eq!(value, "example_value");
    memory_forget.delete_data("example_key");
    assert!(memory_forget.retrieve_data("example_key").is_none());
    loop {}
}

pub struct AIMemoryForget {
    data_store: Vec<(String, String)>,
}

impl AIMemoryForget {
    pub fn new() -> Self {
        AIMemoryForget {
            data_store: Vec::new(),
        }
    }

    pub fn store_data(&mut self, key: &str, value: &str) {
        let entry = (String::from(key), String::from(value));
        self.data_store.push(entry);
    }

    pub fn retrieve_data(&self, key: &str) -> Option<&String> {
        for (k, v) in &self.data_store {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    pub fn delete_data(&mut self, key: &str) {
        self.data_store.retain(|(k, _)| k != key);
    }

    pub fn update_data(&mut self, key: &str, new_value: &str) -> bool {
        for entry in &mut self.data_store {
            if entry.0 == key {
                entry.1 = String::from(new_value);
                return true;
            }
        }
        false
    }

    pub fn list_keys(&self) -> Vec<&String> {
        self.data_store.iter().map(|(k, _)| k).collect()
    }
}
