extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ContextPresentationDetect {
    data: Vec<u8>,
    detected_patterns: Vec<String>,
}

impl ContextPresentationDetect {
    pub fn new() -> Self {
        ContextPresentationDetect {
            data: Vec::new(),
            detected_patterns: Vec::new(),
        }
    }

    pub fn add_data(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }

    pub fn detect_patterns(&mut self) {
        // Example pattern detection logic
        if self.data.contains(&0x41) && self.data.contains(&0x42) {
            self.detected_patterns.push(String::from("Pattern AB detected"));
        }
        if self.data.contains(&0x53) && self.data.contains(&0x54) {
            self.detected_patterns.push(String::from("Pattern ST detected"));
        }
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn get_detected_patterns(&self) -> &[String] {
        &self.detected_patterns
    }

    pub fn reset_detection(&mut self) {
        self.detected_patterns.clear();
    }
}
