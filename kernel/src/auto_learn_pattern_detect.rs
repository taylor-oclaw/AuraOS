extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    println!("Rust module started!");
    0
}

pub struct AutoLearnPatternDetect {
    patterns: Vec<String>,
    detected_patterns: Vec<String>,
}

impl AutoLearnPatternDetect {
    pub fn new() -> Self {
        AutoLearnPatternDetect {
            patterns: Vec::new(),
            detected_patterns: Vec::new(),
        }
    }

    pub fn add_pattern(&mut self, pattern: String) {
        self.patterns.push(pattern);
    }

    pub fn detect_pattern(&mut self, data: &str) -> bool {
        for pattern in &self.patterns {
            if data.contains(pattern) {
                self.detected_patterns.push(pattern.clone());
                return true;
            }
        }
        false
    }

    pub fn get_detected_patterns(&self) -> Vec<String> {
        self.detected_patterns.clone()
    }

    pub fn clear_detected_patterns(&mut self) {
        self.detected_patterns.clear();
    }

    pub fn remove_pattern(&mut self, pattern: &str) -> bool {
        if let Some(index) = self.patterns.iter().position(|p| p == pattern) {
            self.patterns.remove(index);
            true
        } else {
            false
        }
    }
}