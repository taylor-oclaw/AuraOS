extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct MouseKeyLogger {
    keys: Vec<String>,
}

impl MouseKeyLogger {
    pub fn new() -> Self {
        MouseKeyLogger { keys: Vec::new() }
    }

    pub fn log_key(&mut self, key: &str) {
        self.keys.push(key.to_string());
    }

    pub fn get_keys(&self) -> &[String] {
        &self.keys
    }

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    pub fn count_keys(&self) -> usize {
        self.keys.len()
    }

    pub fn find_key(&self, key: &str) -> Option<usize> {
        self.keys.iter().position(|k| k == key)
    }
}
