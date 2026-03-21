extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct NotifyWeekendPattern {
    patterns: Vec<String>,
}

impl NotifyWeekendPattern {
    pub fn new() -> Self {
        NotifyWeekendPattern {
            patterns: Vec::new(),
        }
    }

    pub fn add_pattern(&mut self, pattern: String) {
        self.patterns.push(pattern);
    }

    pub fn remove_pattern(&mut self, index: usize) -> Option<String> {
        if index < self.patterns.len() {
            Some(self.patterns.remove(index))
        } else {
            None
        }
    }

    pub fn get_patterns(&self) -> &[String] {
        &self.patterns
    }

    pub fn clear_patterns(&mut self) {
        self.patterns.clear();
    }

    pub fn matches_pattern(&self, text: &str) -> bool {
        for pattern in &self.patterns {
            if text.contains(pattern) {
                return true;
            }
        }
        false
    }
}
