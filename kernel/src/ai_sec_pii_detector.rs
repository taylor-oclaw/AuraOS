extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct PiiDetector {
    sensitive_words: Vec<String>,
}

impl PiiDetector {
    pub fn new(words: Vec<&str>) -> Self {
        let sensitive_words = words.into_iter().map(|w| w.to_string()).collect();
        PiiDetector { sensitive_words }
    }

    pub fn add_sensitive_word(&mut self, word: &str) {
        if !self.sensitive_words.contains(&word.to_string()) {
            self.sensitive_words.push(word.to_string());
        }
    }

    pub fn remove_sensitive_word(&mut self, word: &str) {
        self.sensitive_words.retain(|w| w != word);
    }

    pub fn contains_pii(&self, text: &str) -> bool {
        for word in &self.sensitive_words {
            if text.contains(word) {
                return true;
            }
        }
        false
    }

    pub fn count_sensitive_words(&self, text: &str) -> usize {
        self.sensitive_words.iter().filter(|word| text.contains(word)).count()
    }

    pub fn list_sensitive_words(&self) -> Vec<String> {
        self.sensitive_words.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pii_detector() {
        let detector = PiiDetector::new(vec!["ssn", "credit card"]);
        assert_eq!(detector.contains_pii("My ssn is 123-45-6789"), true);
        assert_eq!(detector.count_sensitive_words("I have a credit card and an ssn"), 2);
        detector.add_sensitive_word("phone number");
        assert_eq!(detector.list_sensitive_words().len(), 3);
        detector.remove_sensitive_word("ssn");
        assert_eq!(detector.contains_pii("My ssn is 123-45-6789"), false);
    }
}
