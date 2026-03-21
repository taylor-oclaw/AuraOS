extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct LangAccentSpanishMX {
    words: Vec<String>,
}

impl LangAccentSpanishMX {
    pub fn new() -> Self {
        LangAccentSpanishMX {
            words: Vec::new(),
        }
    }

    pub fn add_word(&mut self, word: String) {
        self.words.push(word);
    }

    pub fn get_words(&self) -> &Vec<String> {
        &self.words
    }

    pub fn contains_word(&self, word: &str) -> bool {
        self.words.iter().any(|w| w == word)
    }

    pub fn remove_word(&mut self, word: &str) {
        if let Some(index) = self.words.iter().position(|w| w == word) {
            self.words.remove(index);
        }
    }

    pub fn count_words(&self) -> usize {
        self.words.len()
    }
}
