extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

mod enterprise_dlp_rules {
    use super::*;

    pub struct DLPConfig {
        rules: Vec<String>,
        enabled: bool,
    }

    impl DLPConfig {
        pub fn new() -> Self {
            DLPConfig {
                rules: Vec::new(),
                enabled: false,
            }
        }

        pub fn add_rule(&mut self, rule: String) {
            if !self.rules.contains(&rule) {
                self.rules.push(rule);
            }
        }

        pub fn remove_rule(&mut self, rule: &str) -> bool {
            let index = self.rules.iter().position(|r| r == rule);
            if let Some(idx) = index {
                self.rules.remove(idx);
                true
            } else {
                false
            }
        }

        pub fn enable(&mut self) {
            self.enabled = true;
        }

        pub fn disable(&mut self) {
            self.enabled = false;
        }

        pub fn is_enabled(&self) -> bool {
            self.enabled
        }

        pub fn list_rules(&self) -> Vec<String> {
            self.rules.clone()
        }
    }
}

#[no_mangle]
pub extern "C" fn enterprise_dlp_rules_init() -> i32 {
    0
}