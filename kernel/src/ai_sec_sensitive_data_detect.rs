extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct AISensitiveDataDetector {
    sensitive_words: Vec<String>,
}

impl AISensitiveDataDetector {
    pub fn new(words: Vec<&str>) -> Self {
        let sensitive_words = words.into_iter().map(|w| w.to_string()).collect();
        AISensitiveDataDetector { sensitive_words }
    }

    pub fn add_sensitive_word(&mut self, word: &str) {
        if !self.sensitive_words.contains(&word.to_string()) {
            self.sensitive_words.push(word.to_string());
        }
    }

    pub fn remove_sensitive_word(&mut self, word: &str) {
        self.sensitive_words.retain(|w| w != word);
    }

    pub fn contains_sensitive_data(&self, data: &str) -> bool {
        for word in &self.sensitive_words {
            if data.contains(word) {
                return true;
            }
        }
        false
    }

    pub fn list_sensitive_words(&self) -> Vec<String> {
        self.sensitive_words.clone()
    }

    pub fn count_sensitive_words(&self) -> usize {
        self.sensitive_words.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detector() {
        let detector = AISensitiveDataDetector::new(vec!["password", "secret"]);
        assert_eq!(detector.contains_sensitive_data("This is a secret message"), true);
        assert_eq!(detector.contains_sensitive_data("No sensitive words here"), false);

        detector.add_sensitive_word("token");
        assert_eq!(detector.contains_sensitive_data("Your token is expired"), true);
        assert_eq!(detector.count_sensitive_words(), 3);

        detector.remove_sensitive_word("password");
        assert_eq!(detector.contains_sensitive_data("This is a secret message"), true);
        assert_eq!(detector.count_sensitive_words(), 2);

        let words = detector.list_sensitive_words();
        assert_eq!(words, vec![String::from("secret"), String::from("token")]);
    }
}
