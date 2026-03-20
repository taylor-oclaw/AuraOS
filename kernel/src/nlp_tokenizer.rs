extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct NlpTokenizer {
    // Example field: a list of stop words
    stop_words: Vec<String>,
}

impl NlpTokenizer {
    pub fn new() -> Self {
        NlpTokenizer {
            stop_words: vec![
                String::from("the"),
                String::from("and"),
                String::from("is"),
                String::from("in"),
                String::from("to"),
            ],
        }
    }

    // Method to tokenize a sentence into words
    pub fn tokenize(&self, sentence: &str) -> Vec<String> {
        sentence.split_whitespace()
            .map(|word| word.to_lowercase())
            .filter(|word| !self.stop_words.iter().any(|w| w == word))
            .collect()
    }

    // Method to add a stop word to the tokenizer
    pub fn add_stop_word(&mut self, word: String) {
        if !self.stop_words.contains(&word) {
            self.stop_words.push(word);
        }
    }

    // Method to remove a stop word from the tokenizer
    pub fn remove_stop_word(&mut self, word: &str) {
        self.stop_words.retain(|w| w != word);
    }

    // Method to check if a word is a stop word
    pub fn is_stop_word(&self, word: &str) -> bool {
        self.stop_words.iter().any(|w| w == word)
    }

    // Method to get the list of stop words
    pub fn get_stop_words(&self) -> Vec<String> {
        self.stop_words.clone()
    }
}
