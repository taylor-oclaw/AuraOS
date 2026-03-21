extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct AISensitiveDataClassifier {
    // Example data structure to hold sensitive patterns or rules
    patterns: Vec<String>,
}

impl AISensitiveDataClassifier {
    pub fn new(patterns: Vec<String>) -> Self {
        AISensitiveDataClassifier { patterns }
    }

    pub fn add_pattern(&mut self, pattern: String) {
        self.patterns.push(pattern);
    }

    pub fn remove_pattern(&mut self, pattern: &str) -> bool {
        if let Some(index) = self.patterns.iter().position(|p| p == pattern) {
            self.patterns.remove(index);
            true
        } else {
            false
        }
    }

    pub fn contains_pattern(&self, data: &str) -> bool {
        for pattern in &self.patterns {
            if data.contains(pattern) {
                return true;
            }
        }
        false
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
    fn test_classifier() {
        let mut classifier = AISensitiveDataClassifier::new(vec![
            String::from("password"),
            String::from("secret"),
        ];

        assert!(classifier.contains_pattern("This is a secret message"));
        assert!(!classifier.contains_pattern("This is a public message"));

        classifier.add_pattern(String::from("confidential"));
        assert!(classifier.contains_pattern("This is confidential information"));

        classifier.remove_pattern("password");
        assert!(!classifier.contains_pattern("This is a password"));

        let patterns = classifier.list_patterns();
        assert_eq!(patterns.len(), 2);
        assert!(patterns.contains(&String::from("secret")));
        assert!(patterns.contains(&String::from("confidential")));

        classifier.clear_patterns();
        assert!(!classifier.contains_pattern("This is confidential information"));
    }
}
