extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn security_typing_pattern_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn security_typing_pattern_exit() {
    // Cleanup logic for the module
}

pub struct TypingPattern {
    patterns: Vec<String>,
}

impl TypingPattern {
    pub fn new() -> Self {
        TypingPattern {
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

    pub fn clear_patterns(&mut self) {
        self.patterns.clear();
    }

    pub fn pattern_exists(&self, pattern: &str) -> bool {
        self.patterns.iter().any(|p| p == pattern)
    }
}
