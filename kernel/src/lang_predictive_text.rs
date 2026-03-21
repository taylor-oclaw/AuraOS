extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct LangPredictiveText {
    // Example data structure for simplicity
    words: Vec<String>,
}

impl LangPredictiveText {
    pub fn new() -> Self {
        LangPredictiveText {
            words: Vec::new(),
        }
    }

    pub fn add_word(&mut self, word: String) {
        self.words.push(word);
    }

    pub fn predict_next(&self, prefix: &str) -> Option<String> {
        for word in &self.words {
            if word.starts_with(prefix) {
                return Some(word.clone());
            }
        }
        None
    }

    pub fn remove_word(&mut self, word: &str) -> bool {
        let pos = self.words.iter().position(|w| w == word);
        match pos {
            Some(index) => {
                self.words.remove(index);
                true
            },
            None => false,
        }
    }

    pub fn list_words(&self) -> Vec<String> {
        self.words.clone()
    }

    pub fn count_words(&self) -> usize {
        self.words.len()
    }
}
