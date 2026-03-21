extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct NotifyLearnSnoozePattern {
    patterns: Vec<String>,
    current_pattern_index: usize,
}

impl NotifyLearnSnoozePattern {
    pub fn new(patterns: Vec<String>) -> Self {
        NotifyLearnSnoozePattern {
            patterns,
            current_pattern_index: 0,
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

    pub fn get_current_pattern(&self) -> Option<&String> {
        self.patterns.get(self.current_pattern_index)
    }

    pub fn next_pattern(&mut self) -> Option<&String> {
        if self.current_pattern_index < self.patterns.len() - 1 {
            self.current_pattern_index += 1;
            Some(&self.patterns[self.current_pattern_index])
        } else {
            None
        }
    }

    pub fn previous_pattern(&mut self) -> Option<&String> {
        if self.current_pattern_index > 0 {
            self.current_pattern_index -= 1;
            Some(&self.patterns[self.current_pattern_index])
        } else {
            None
        }
    }
}
