extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn security_usage_pattern_init() {
    // Initialization logic for the module
}

pub extern "C" fn security_usage_pattern_exit() {
    // Cleanup logic for the module
}

pub struct SecurityUsagePattern {
    patterns: Vec<String>,
}

impl SecurityUsagePattern {
    pub fn new() -> Self {
        SecurityUsagePattern {
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

    pub fn find_pattern(&self, pattern: &str) -> Option<usize> {
        self.patterns.iter().position(|p| p == pattern)
    }

    pub fn clear_patterns(&mut self) {
        self.patterns.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_usage_pattern() {
        let mut usage = SecurityUsagePattern::new();

        assert_eq!(usage.get_patterns().len(), 0);

        usage.add_pattern(String::from("pattern1"));
        usage.add_pattern(String::from("pattern2"));

        assert_eq!(usage.get_patterns().len(), 2);
        assert_eq!(usage.find_pattern("pattern1"), Some(0));
        assert_eq!(usage.find_pattern("pattern3"), None);

        let removed = usage.remove_pattern(0);
        assert_eq!(removed, Some(String::from("pattern1")));
        assert_eq!(usage.get_patterns().len(), 1);

        usage.clear_patterns();
        assert_eq!(usage.get_patterns().len(), 0);
    }
}
