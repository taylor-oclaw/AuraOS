extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialize the module
}

pub extern "C" fn rust_ffi_exit() {
    // Clean up the module
}

pub struct AuraFirewall {
    rules: Vec<String>,
}

impl AuraFirewall {
    pub fn new() -> Self {
        AuraFirewall { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: String) {
        self.rules.push(rule);
    }

    pub fn remove_rule(&mut self, index: usize) -> Option<String> {
        if index < self.rules.len() {
            Some(self.rules.remove(index))
        } else {
            None
        }
    }

    pub fn list_rules(&self) -> Vec<String> {
        self.rules.clone()
    }

    pub fn is_allowed(&self, packet: &str) -> bool {
        for rule in &self.rules {
            if packet.contains(rule) {
                return false;
            }
        }
        true
    }

    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }
}
