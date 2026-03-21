extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PredictAppContextPattern {
    patterns: Vec<String>,
}

impl PredictAppContextPattern {
    pub fn new() -> Self {
        PredictAppContextPattern {
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

    pub fn get_pattern(&self, index: usize) -> Option<&String> {
        self.patterns.get(index)
    }

    pub fn contains_pattern(&self, pattern: &str) -> bool {
        self.patterns.iter().any(|p| p == pattern)
    }

    pub fn list_patterns(&self) -> &[String] {
        &self.patterns
    }
}
