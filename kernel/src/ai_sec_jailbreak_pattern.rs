extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn init_module() -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn cleanup_module() {}

struct JailbreakPatternDetector {
    patterns: Vec<String>,
}

impl JailbreakPatternDetector {
    pub fn new(patterns: Vec<&str>) -> Self {
        JailbreakPatternDetector {
            patterns: patterns.into_iter().map(|p| p.to_string()).collect(),
        }
    }

    pub fn add_pattern(&mut self, pattern: &str) {
        self.patterns.push(pattern.to_string());
    }

    pub fn remove_pattern(&mut self, pattern: &str) -> bool {
        if let Some(index) = self.patterns.iter().position(|p| p == pattern) {
            self.patterns.remove(index);
            true
        } else {
            false
        }
    }

    pub fn contains_pattern(&self, text: &str) -> bool {
        self.patterns.iter().any(|pattern| text.contains(pattern))
    }

    pub fn list_patterns(&self) -> Vec<String> {
        self.patterns.clone()
    }

    pub fn clear_patterns(&mut self) {
        self.patterns.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jailbreak_pattern_detector() {
        let mut detector = JailbreakPatternDetector::new(vec!["pattern1", "pattern2"]);
        assert_eq!(detector.contains_pattern("This is pattern1"), true);
        assert_eq!(detector.contains_pattern("This is pattern3"), false);

        detector.add_pattern("pattern3");
        assert_eq!(detector.contains_pattern("This is pattern3"), true);

        let removed = detector.remove_pattern("pattern2");
        assert_eq!(removed, true);
        assert_eq!(detector.contains_pattern("This is pattern2"), false);

        let patterns = detector.list_patterns();
        assert_eq!(patterns.len(), 2);
        assert_eq!(patterns.contains(&String::from("pattern1")), true);
        assert_eq!(patterns.contains(&String::from("pattern3")), true);

        detector.clear_patterns();
        assert_eq!(detector.contains_pattern("This is pattern1"), false);
        assert_eq!(detector.contains_pattern("This is pattern3"), false);
    }
}
