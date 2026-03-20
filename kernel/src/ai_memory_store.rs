extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut store = ai_memory_store::new();
    store.store_data(String::from("key1"), String::from("value1"));
    store.store_data(String::from("key2"), String::from("value2"));

    if let Some(value) = store.retrieve_data(String::from("key1")) {
        // Do something with the retrieved value
    }

    loop {}
}

mod ai_memory_store {
    use super::*;

    pub struct ai_memory_store {
        data: Vec<(String, String)>,
    }

    impl ai_memory_store {
        pub fn new() -> Self {
            ai_memory_store { data: Vec::new() }
        }

        pub fn store_data(&mut self, key: String, value: String) {
            for (k, v) in &mut self.data {
                if *k == key {
                    *v = value;
                    return;
                }
            }
            self.data.push((key, value));
        }

        pub fn retrieve_data(&self, key: String) -> Option<String> {
            for (k, v) in &self.data {
                if *k == key {
                    return Some(v.clone());
                }
            }
            None
        }

        pub fn delete_data(&mut self, key: String) {
            self.data.retain(|(k, _)| *k != key);
        }

        pub fn update_data(&mut self, key: String, new_value: String) -> bool {
            for (k, v) in &mut self.data {
                if *k == key {
                    *v = new_value;
                    return true;
                }
            }
            false
        }

        pub fn list_keys(&self) -> Vec<String> {
            self.data.iter().map(|(k, _)| k.clone()).collect()
        }
    }
}
