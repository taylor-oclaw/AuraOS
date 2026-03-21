extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct ClickPatternDetector {
    patterns: Vec<Vec<u8>>,
    click_sequence: Vec<u8>,
}

impl ClickPatternDetector {
    pub fn new() -> Self {
        ClickPatternDetector {
            patterns: Vec::new(),
            click_sequence: Vec::new(),
        }
    }

    pub fn add_pattern(&mut self, pattern: Vec<u8>) {
        self.patterns.push(pattern);
    }

    pub fn record_click(&mut self, click: u8) {
        self.click_sequence.push(click);
    }

    pub fn reset_sequence(&mut self) {
        self.click_sequence.clear();
    }

    pub fn detect_pattern(&self) -> Option<usize> {
        for (index, pattern) in self.patterns.iter().enumerate() {
            if self.click_sequence.len() >= pattern.len()
                && &self.click_sequence[self.click_sequence.len() - pattern.len()..] == pattern
            {
                return Some(index);
            }
        }
        None
    }

    pub fn get_patterns(&self) -> Vec<Vec<u8>> {
        self.patterns.clone()
    }
}
