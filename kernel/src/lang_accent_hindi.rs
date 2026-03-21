extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn lang_accent_hindi_init() {
    // Initialization logic for the Hindi language accent module
}

pub extern "C" fn lang_accent_hindi_exit() {
    // Cleanup logic for the Hindi language accent module
}

pub struct HindiAccent {
    words: Vec<String>,
    rules: Vec<(String, String)>, // (original, accented)
}

impl HindiAccent {
    pub fn new() -> Self {
        HindiAccent {
            words: Vec::new(),
            rules: vec![
                (String::from("a"), String::from("आ")),
                (String::from("i"), String::from("इ")),
                (String::from("u"), String::from("उ")),
                (String::from("e"), String::from("ए")),
                (String::from("o"), String::from("ओ")),
            ],
        }
    }

    pub fn add_word(&mut self, word: String) {
        self.words.push(word);
    }

    pub fn get_words(&self) -> &Vec<String> {
        &self.words
    }

    pub fn accent_word(&self, word: &str) -> Option<String> {
        let mut accented = word.to_string();
        for (original, accented_char) in &self.rules {
            accented = accented.replace(original, accented_char);
        }
        if accented != word {
            Some(accented)
        } else {
            None
        }
    }

    pub fn accent_all_words(&self) -> Vec<String> {
        self.words.iter().filter_map(|word| self.accent_word(word)).collect()
    }

    pub fn clear_words(&mut self) {
        self.words.clear();
    }
}
