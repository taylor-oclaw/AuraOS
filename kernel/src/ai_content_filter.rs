extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn ai_content_filter_init() {
    // Initialization logic for the AI content filter module
}

pub extern "C" fn ai_content_filter_exit() {
    // Cleanup logic for the AI content filter module
}

pub struct ContentFilter {
    blocked_words: Vec<String>,
    allowed_words: Vec<String>,
}

impl ContentFilter {
    pub fn new() -> Self {
        ContentFilter {
            blocked_words: Vec::new(),
            allowed_words: Vec::new(),
        }
    }

    pub fn add_blocked_word(&mut self, word: String) {
        self.blocked_words.push(word);
    }

    pub fn remove_blocked_word(&mut self, word: &str) -> bool {
        if let Some(index) = self.blocked_words.iter().position(|w| w == word) {
            self.blocked_words.remove(index);
            true
        } else {
            false
        }
    }

    pub fn add_allowed_word(&mut self, word: String) {
        self.allowed_words.push(word);
    }

    pub fn remove_allowed_word(&mut self, word: &str) -> bool {
        if let Some(index) = self.allowed_words.iter().position(|w| w == word) {
            self.allowed_words.remove(index);
            true
        } else {
            false
        }
    }

    pub fn is_content_allowed(&self, content: &str) -> bool {
        for allowed_word in &self.allowed_words {
            if content.contains(allowed_word) {
                return true;
            }
        }
        for blocked_word in &self.blocked_words {
            if content.contains(blocked_word) {
                return false;
            }
        }
        true
    }
}
