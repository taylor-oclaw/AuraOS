extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_accent_english_us_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn lang_accent_english_us_exit() {
    // Cleanup logic for the module
}

pub struct EnglishUSAccent {
    words: Vec<String>,
}

impl EnglishUSAccent {
    pub fn new() -> Self {
        EnglishUSAccent {
            words: Vec::new(),
        }
    }

    pub fn add_word(&mut self, word: String) {
        self.words.push(word);
    }

    pub fn get_words(&self) -> &Vec<String> {
        &self.words
    }

    pub fn remove_word(&mut self, index: usize) -> Option<String> {
        if index < self.words.len() {
            Some(self.words.remove(index))
        } else {
            None
        }
    }

    pub fn count_words(&self) -> usize {
        self.words.len()
    }

    pub fn find_word(&self, word: &str) -> Option<usize> {
        self.words.iter().position(|w| w == word)
    }
}
