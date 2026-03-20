extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut memory = AgentMemoryWorking::new();
    memory.store_data("key1", "value1");
    memory.store_data("key2", "value2");
    if let Some(value) = memory.retrieve_data("key1") {
        println!("Retrieved value: {}", value);
    }
    memory.delete_data("key2");
    if let Some(count) = memory.data_count() {
        println!("Data count: {}", count);
    }
}

pub struct AgentMemoryWorking {
    data_store: Vec<(String, String)>,
}

impl AgentMemoryWorking {
    pub fn new() -> Self {
        AgentMemoryWorking {
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

    pub fn data_count(&self) -> Option<usize> {
        Some(self.data_store.len())
    }

    pub fn clear_all(&mut self) {
        self.data_store.clear();
    }
}
