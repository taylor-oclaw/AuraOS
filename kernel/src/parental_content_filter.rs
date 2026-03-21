extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ParentalContentFilter {
    blocked_words: Vec<String>,
}

impl ParentalContentFilter {
    pub fn new() -> Self {
        ParentalContentFilter {
            blocked_words: Vec::new(),
        }
    }

    pub fn add_blocked_word(&mut self, word: String) {
        if !self.blocked_words.contains(&word) {
            self.blocked_words.push(word);
        }
    }

    pub fn remove_blocked_word(&mut self, word: &str) -> bool {
        let index = self.blocked_words.iter().position(|w| w == word);
        if let Some(i) = index {
            self.blocked_words.remove(i);
            true
        } else {
            false
        }
    }

    pub fn is_blocked(&self, content: &str) -> bool {
        for blocked_word in &self.blocked_words {
            if content.contains(blocked_word) {
                return true;
            }
        }
        false
    }

    pub fn list_blocked_words(&self) -> Vec<String> {
        self.blocked_words.clone()
    }

    pub fn clear_all_blocked_words(&mut self) {
        self.blocked_words.clear();
    }
}
