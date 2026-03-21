extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct LangAutocorrect {
    corrections: Vec<(String, String)>,
}

impl LangAutocorrect {
    pub fn new() -> Self {
        LangAutocorrect {
            corrections: Vec::new(),
        }
    }

    pub fn add_correction(&mut self, misspelled: &str, corrected: &str) {
        let misspelled = String::from(misspelled);
        let corrected = String::from(corrected);
        self.corrections.push((misspelled, corrected));
    }

    pub fn correct_word(&self, word: &str) -> Option<String> {
        for (misspelled, corrected) in &self.corrections {
            if misspelled == word {
                return Some(corrected.clone());
            }
        }
        None
    }

    pub fn correct_sentence(&self, sentence: &str) -> String {
        let words: Vec<&str> = sentence.split_whitespace().collect();
        let mut corrected_words = Vec::new();

        for word in words {
            if let Some(corrected_word) = self.correct_word(word) {
                corrected_words.push(corrected_word);
            } else {
                corrected_words.push(String::from(word));
            }
        }

        corrected_words.join(" ")
    }

    pub fn list_corrections(&self) -> Vec<(String, String)> {
        self.corrections.clone()
    }

    pub fn remove_correction(&mut self, misspelled: &str) {
        self.corrections.retain(|(m, _)| m != misspelled);
    }
}
