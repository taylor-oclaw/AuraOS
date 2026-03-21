extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct FilipinoLanguage {
    words: Vec<String>,
}

impl FilipinoLanguage {
    pub fn new() -> Self {
        FilipinoLanguage {
            words: Vec::new(),
        }
    }

    pub fn add_word(&mut self, word: &str) {
        self.words.push(String::from(word));
    }

    pub fn remove_word(&mut self, word: &str) {
        if let Some(index) = self.words.iter().position(|w| w == word) {
            self.words.remove(index);
        }
    }

    pub fn contains_word(&self, word: &str) -> bool {
        self.words.contains(&String::from(word))
    }

    pub fn get_words_count(&self) -> usize {
        self.words.len()
    }

    pub fn list_words(&self) -> Vec<String> {
        self.words.clone()
    }
}
