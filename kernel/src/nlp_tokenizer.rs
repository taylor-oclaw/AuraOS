extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct NlpTokenizer {
    vocabulary: Vec<String>,
}

impl NlpTokenizer {
    pub fn new(vocabulary: Vec<String>) -> Self {
        NlpTokenizer { vocabulary }
    }

    pub fn tokenize(&self, text: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let words = text.split_whitespace();
        for word in words {
            if self.vocabulary.contains(&word.to_string()) {
                tokens.push(word.to_string());
            } else {
                // Handle unknown words (e.g., subword tokenization, BPE)
                tokens.extend(self.handle_unknown_word(word));
            }
        }
        tokens
    }

    fn handle_unknown_word(&self, word: &str) -> Vec<String> {
        // Simple heuristic: split by characters
        let mut sub_tokens = Vec::new();
        for ch in word.chars() {
            sub_tokens.push(ch.to_string());
        }
        sub_tokens
    }

    pub fn add_to_vocabulary(&mut self, word: String) {
        if !self.vocabulary.contains(&word) {
            self.vocabulary.push(word);
        }
    }

    pub fn remove_from_vocabulary(&mut self, word: &str) {
        if let Some(index) = self.vocabulary.iter().position(|w| w == word) {
            self.vocabulary.remove(index);
        }
    }

    pub fn vocabulary_size(&self) -> usize {
        self.vocabulary.len()
    }
}
