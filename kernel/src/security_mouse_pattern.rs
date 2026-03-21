extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn security_mouse_pattern_init() {
    // Initialization logic for the module
}

pub extern "C" fn security_mouse_pattern_exit() {
    // Cleanup logic for the module
}

pub struct MousePattern {
    pattern: Vec<u8>,
    threshold: u8,
}

impl MousePattern {
    pub fn new(pattern: Vec<u8>, threshold: u8) -> Self {
        MousePattern { pattern, threshold }
    }

    pub fn set_pattern(&mut self, pattern: Vec<u8>) {
        self.pattern = pattern;
    }

    pub fn get_pattern(&self) -> &Vec<u8> {
        &self.pattern
    }

    pub fn set_threshold(&mut self, threshold: u8) {
        self.threshold = threshold;
    }

    pub fn get_threshold(&self) -> u8 {
        self.threshold
    }

    pub fn match_pattern(&self, input: &[u8]) -> bool {
        if input.len() != self.pattern.len() {
            return false;
        }
        let mut diff_count = 0;
        for (a, b) in self.pattern.iter().zip(input.iter()) {
            if a != b {
                diff_count += 1;
                if diff_count > self.threshold {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mouse_pattern() {
        let pattern = vec![1, 2, 3, 4];
        let threshold = 1;
        let mut mp = MousePattern::new(pattern.clone(), threshold);

        assert_eq!(mp.get_pattern(), &pattern);
        assert_eq!(mp.get_threshold(), threshold);

        let test_input = vec![1, 2, 3, 5];
        assert!(mp.match_pattern(&test_input));

        let test_input = vec![1, 2, 4, 5];
        assert!(!mp.match_pattern(&test_input));

        mp.set_threshold(2);
        assert!(mp.match_pattern(&test_input));
    }
}
