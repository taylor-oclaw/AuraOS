extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct AIJailbreakDetector {
    patterns: Vec<String>,
    detected_patterns: Vec<String>,
}

impl AIJailbreakDetector {
    pub fn new(patterns: Vec<&str>) -> Self {
        AIJailbreakDetector {
            patterns: patterns.into_iter().map(|p| p.to_string()).collect(),
            detected_patterns: Vec::new(),
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

    pub fn scan_text(&mut self, text: &str) {
        for pattern in &self.patterns {
            if text.contains(pattern) {
                self.detected_patterns.push(pattern.clone());
            }
        }
    }

    pub fn get_detected_patterns(&self) -> Vec<String> {
        self.detected_patterns.clone()
    }

    pub fn clear_detected_patterns(&mut self) {
        self.detected_patterns.clear();
    }
}
