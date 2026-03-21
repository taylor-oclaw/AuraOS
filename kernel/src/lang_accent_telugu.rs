extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_accent_telugu_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn lang_accent_telugu_exit() {
    // Cleanup logic for the module
}

pub struct TeluguAccent {
    words: Vec<String>,
}

impl TeluguAccent {
    pub fn new() -> Self {
        TeluguAccent {
            words: Vec::new(),
        }
    }

    pub fn add_word(&mut self, word: &str) {
        self.words.push(String::from(word));
    }

    pub fn remove_word(&mut self, word: &str) -> bool {
        if let Some(index) = self.words.iter().position(|w| w == word) {
            self.words.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_words(&self) -> Vec<String> {
        self.words.clone()
    }

    pub fn contains_word(&self, word: &str) -> bool {
        self.words.contains(&String::from(word))
    }

    pub fn count_words(&self) -> usize {
        self.words.len()
    }
}
