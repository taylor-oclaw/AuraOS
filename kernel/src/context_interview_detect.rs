extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct ContextInterviewDetect {
    data: Vec<u8>,
    processed_data: Vec<u8>,
    detected_patterns: Vec<String>,
}

impl ContextInterviewDetect {
    pub fn new() -> Self {
        ContextInterviewDetect {
            data: Vec::new(),
            processed_data: Vec::new(),
            detected_patterns: Vec::new(),
        }
    }

    pub fn add_data(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }

    pub fn process_data(&mut self) {
        // Simple processing: reverse the data
        self.processed_data = self.data.iter().cloned().rev().collect();
    }

    pub fn detect_patterns(&mut self) {
        // Dummy pattern detection logic
        let patterns = vec!["AI", "kernel", "module"];
        for pattern in patterns {
            if self.contains_pattern(pattern) {
                self.detected_patterns.push(String::from(pattern));
            }
        }
    }

    pub fn contains_pattern(&self, pattern: &str) -> bool {
        // Check if the processed data contains the given pattern
        let pattern_bytes = pattern.as_bytes();
        for window in self.processed_data.windows(pattern_bytes.len()) {
            if window == pattern_bytes {
                return true;
            }
        }
        false
    }

    pub fn get_detected_patterns(&self) -> &[String] {
        &self.detected_patterns
    }
}
