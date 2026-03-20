extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn seccomp_filter_init() {
    // Initialization logic for the seccomp filter module
}

#[no_mangle]
pub extern "C" fn seccomp_filter_exit() {
    // Cleanup logic for the seccomp filter module
}

pub struct SeccompFilter {
    rules: Vec<String>,
}

impl SeccompFilter {
    pub fn new() -> Self {
        SeccompFilter { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: String) {
        self.rules.push(rule);
    }

    pub fn remove_rule(&mut self, rule: &str) {
        if let Some(index) = self.rules.iter().position(|r| r == rule) {
            self.rules.remove(index);
        }
    }

    pub fn has_rule(&self, rule: &str) -> bool {
        self.rules.contains(&String::from(rule))
    }

    pub fn list_rules(&self) -> Vec<String> {
        self.rules.clone()
    }

    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }
}
