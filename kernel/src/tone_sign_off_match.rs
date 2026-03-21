extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ToneSignOffMatch {
    patterns: Vec<String>,
}

impl ToneSignOffMatch {
    pub fn new() -> Self {
        ToneSignOffMatch {
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

    pub fn get_patterns(&self) -> &Vec<String> {
        &self.patterns
    }

    pub fn match_pattern(&self, text: &str) -> bool {
        for pattern in &self.patterns {
            if text.contains(pattern) {
                return true;
            }
        }
        false
    }

    pub fn clear_patterns(&mut self) {
        self.patterns.clear();
    }
}
