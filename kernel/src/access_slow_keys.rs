extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut slow_keys = SlowKeys::new();
    slow_keys.add_key("slow_key1");
    slow_keys.add_key("slow_key2");
    slow_keys.add_key("slow_key3");

    if slow_keys.contains_key("slow_key2") {
        println!("Key found!");
    } else {
        println!("Key not found.");
    }

    let keys = slow_keys.get_all_keys();
    for key in keys.iter() {
        println!("Key: {}", key);
    }

    slow_keys.remove_key("slow_key1");
    if !slow_keys.contains_key("slow_key1") {
        println!("Key removed successfully.");
    } else {
        println!("Failed to remove key.");
    }
}

pub struct SlowKeys {
    keys: Vec<String>,
}

impl SlowKeys {
    pub fn new() -> Self {
        SlowKeys { keys: Vec::new() }
    }

    pub fn add_key(&mut self, key: &str) {
        if !self.contains_key(key) {
            self.keys.push(String::from(key));
        }
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.keys.iter().any(|k| k == key)
    }

    pub fn get_all_keys(&self) -> Vec<String> {
        self.keys.clone()
    }

    pub fn remove_key(&mut self, key: &str) {
        if let Some(index) = self.keys.iter().position(|k| k == key) {
            self.keys.remove(index);
        }
    }
}
