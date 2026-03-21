extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn lang_spellcheck_init() {
    // Initialization logic for the spell checker module
}

pub extern "C" fn lang_spellcheck_exit() {
    // Cleanup logic for the spell checker module
}

pub struct SpellChecker {
    dictionary: Vec<String>,
}

impl SpellChecker {
    pub fn new(dictionary: Vec<String>) -> Self {
        SpellChecker { dictionary }
    }

    pub fn add_word(&mut self, word: String) {
        if !self.dictionary.contains(&word) {
            self.dictionary.push(word);
        }
    }

    pub fn remove_word(&mut self, word: &str) {
        self.dictionary.retain(|w| w != word);
    }

    pub fn is_correct(&self, word: &str) -> bool {
        self.dictionary.contains(&String::from(word))
    }

    pub fn suggest_corrections(&self, misspelled_word: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        for word in &self.dictionary {
            if Self::levenshtein_distance(misspelled_word, word) <= 2 {
                suggestions.push(word.clone());
            }
        }
        suggestions
    }

    fn levenshtein_distance(s1: &str, s2: &str) -> usize {
        let mut d = vec![vec![0; s2.len() + 1]; s1.len() + 1];

        for i in 0..=s1.len() {
            d[i][0] = i;
        }
        for j in 0..=s2.len() {
            d[0][j] = j;
        }

        for (i, c1) in s1.chars().enumerate() {
            for (j, c2) in s2.chars().enumerate() {
                let cost = if c1 == c2 { 0 } else { 1 };
                d[i + 1][j + 1] = core::cmp::min(
                    core::cmp::min(d[i][j + 1] + 1, d[i + 1][j] + 1),
                    d[i][j] + cost,
                ;
            }
        }

        d[s1.len()][s2.len()]
    }
}
