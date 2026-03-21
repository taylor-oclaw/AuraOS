extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn init_module() -> i32 {
    0
}

pub extern "C" fn cleanup_module() -> i32 {
    0
}

pub struct CanaryModule {
    data: Vec<u8>,
    name: String,
    version: u32,
    enabled: bool,
}

impl CanaryModule {
    pub fn new(name: &str, version: u32) -> Self {
        CanaryModule {
            data: Vec::new(),
            name: String::from(name),
            version,
            enabled: true,
        }
    }

    pub fn add_data(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn toggle_enabled(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
