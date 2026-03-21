extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    let mut detector = NotifyCodingDetect::new();
    detector.add_pattern("AI".to_string());
    detector.add_pattern("Rust".to_string());

    if detector.detect("This is a Rust coding session.") {
        detector.notify("Rust detected!");
    } else {
        detector.notify("No specific pattern found.");
    }
}

pub struct NotifyCodingDetect {
    patterns: Vec<String>,
}

impl NotifyCodingDetect {
    pub fn new() -> Self {
        NotifyCodingDetect {
            patterns: Vec::new(),
        }
    }

    pub fn add_pattern(&mut self, pattern: String) {
        self.patterns.push(pattern);
    }

    pub fn remove_pattern(&mut self, pattern: &str) -> bool {
        if let Some(index) = self.patterns.iter().position(|p| p == pattern) {
            self.patterns.remove(index);
            true
        } else {
            false
        }
    }

    pub fn detect(&self, text: &str) -> bool {
        for pattern in &self.patterns {
            if text.contains(pattern) {
                return true;
            }
        }
        false
    }

    pub fn notify(&self, message: &str) {
        // Simulate a notification mechanism
    }

    pub fn list_patterns(&self) -> Vec<String> {
        self.patterns.clone()
    }
}
