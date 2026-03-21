extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct NotifyLearnDismissPattern {
    patterns: Vec<String>,
}

impl NotifyLearnDismissPattern {
    pub fn new() -> Self {
        NotifyLearnDismissPattern {
            patterns: Vec::new(),
        }
    }

    pub fn add_pattern(&mut self, pattern: String) {
        if !self.patterns.contains(&pattern) {
            self.patterns.push(pattern);
        }
    }

    pub fn remove_pattern(&mut self, pattern: &str) -> bool {
        let index = self.patterns.iter().position(|p| p == pattern);
        match index {
            Some(i) => {
                self.patterns.remove(i);
                true
            }
            None => false,
        }
    }

    pub fn contains_pattern(&self, pattern: &str) -> bool {
        self.patterns.contains(&String::from(pattern))
    }

    pub fn list_patterns(&self) -> Vec<String> {
        self.patterns.clone()
    }

    pub fn clear_patterns(&mut self) {
        self.patterns.clear();
    }
}
