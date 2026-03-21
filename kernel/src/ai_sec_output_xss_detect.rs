extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct XSSDetector {
    patterns: Vec<String>,
}

impl XSSDetector {
    pub fn new(patterns: Vec<&str>) -> Self {
        XSSDetector {
            patterns: patterns.into_iter().map(|p| p.to_string()).collect(),
        }
    }

    pub fn add_pattern(&mut self, pattern: &str) {
        self.patterns.push(pattern.to_string());
    }

    pub fn remove_pattern(&mut self, pattern: &str) -> bool {
        if let Some(index) = self.patterns.iter().position(|p| p == pattern) {
            self.patterns.remove(index);
            true
        } else {
            false
        }
    }

    pub fn detect(&self, input: &str) -> bool {
        for pattern in &self.patterns {
            if input.contains(pattern) {
                return true;
            }
        }
        false
    }

    pub fn list_patterns(&self) -> Vec<String> {
        self.patterns.clone()
    }

    pub fn clear_patterns(&mut self) {
        self.patterns.clear();
    }
}
