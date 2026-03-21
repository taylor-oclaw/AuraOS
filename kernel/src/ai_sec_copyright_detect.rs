extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct AICopyrightDetector {
    // Example fields, replace with actual logic
    database: Vec<String>,
    detected_copyrights: Vec<String>,
}

impl AICopyrightDetector {
    pub fn new() -> Self {
        AICopyrightDetector {
            database: Vec::new(),
            detected_copyrights: Vec::new(),
        }
    }

    pub fn add_to_database(&mut self, copyright: String) {
        self.database.push(copyright);
    }

    pub fn scan_text(&mut self, text: &str) {
        for line in text.lines() {
            if self.is_copyright_protected(line) {
                self.detected_copyrights.push(line.to_string());
            }
        }
    }

    fn is_copyright_protected(&self, line: &str) -> bool {
        // Simple heuristic check for copyright string
        line.contains("Copyright") || line.contains("©")
    }

    pub fn get_detected_copyrights(&self) -> &[String] {
        &self.detected_copyrights
    }

    pub fn clear_detected_copyrights(&mut self) {
        self.detected_copyrights.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_to_database() {
        let mut detector = AICopyrightDetector::new();
        detector.add_to_database("Copyright 2023".to_string());
        assert_eq!(detector.database.len(), 1);
    }

    #[test]
    fn test_scan_text() {
        let mut detector = AICopyrightDetector::new();
        detector.scan_text("This is a test. Copyright 2023.");
        assert_eq!(detector.get_detected_copyrights().len(), 1);
    }

    #[test]
    fn test_clear_detected_copyrights() {
        let mut detector = AICopyrightDetector::new();
        detector.scan_text("This is a test. © 2023.");
        detector.clear_detected_copyrights();
        assert_eq!(detector.get_detected_copyrights().len(), 0);
    }
}
