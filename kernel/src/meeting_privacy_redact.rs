extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn meeting_privacy_redact_init() -> i32 {
    0
}

pub extern "C" fn meeting_privacy_redact_exit() -> i32 {
    0
}

pub struct MeetingPrivacyRedactor {
    redacted_words: Vec<String>,
}

impl MeetingPrivacyRedactor {
    pub fn new() -> Self {
        MeetingPrivacyRedactor {
            redacted_words: Vec::new(),
        }
    }

    pub fn add_redacted_word(&mut self, word: &str) {
        self.redacted_words.push(word.to_string());
    }

    pub fn remove_redacted_word(&mut self, word: &str) -> bool {
        if let Some(index) = self.redacted_words.iter().position(|w| w == word) {
            self.redacted_words.remove(index);
            true
        } else {
            false
        }
    }

    pub fn is_redacted_word(&self, word: &str) -> bool {
        self.redacted_words.contains(&word.to_string())
    }

    pub fn redact_text(&self, text: &str) -> String {
        let mut result = String::new();
        for word in text.split_whitespace() {
            if self.is_redacted_word(word) {
                result.push_str("[REDACTED]");
            } else {
                result.push_str(word);
            }
            result.push(' ');
        }
        if !result.is_empty() {
            result.pop(); // Remove the trailing space
        }
        result
    }

    pub fn list_redacted_words(&self) -> Vec<String> {
        self.redacted_words.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_remove_redacted_word() {
        let mut redactor = MeetingPrivacyRedactor::new();
        assert!(!redactor.is_redacted_word("secret"));
        redactor.add_redacted_word("secret");
        assert!(redactor.is_redacted_word("secret"));
        assert!(redactor.remove_redacted_word("secret"));
        assert!(!redactor.is_redacted_word("secret"));
    }

    #[test]
    fn test_redact_text() {
        let mut redactor = MeetingPrivacyRedactor::new();
        redactor.add_redacted_word("password");
        redactor.add_redacted_word("secret");

        let text = "This is a secret password";
        let redacted_text = redactor.redact_text(text);
        assert_eq!(redacted_text, "This is a [REDACTED] [REDACTED]");
    }

    #[test]
    fn test_list_redacted_words() {
        let mut redactor = MeetingPrivacyRedactor::new();
        redactor.add_redacted_word("password");
        redactor.add_redacted_word("secret");

        let words = redactor.list_redacted_words();
        assert_eq!(words, vec![String::from("password"), String::from("secret")]);
    }
}
