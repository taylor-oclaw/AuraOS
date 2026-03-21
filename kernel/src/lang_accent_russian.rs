extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn lang_accent_russian_init() {
    // Initialization logic for the module
}

pub extern "C" fn lang_accent_russian_exit() {
    // Cleanup logic for the module
}

pub struct RussianAccent {
    words: Vec<String>,
}

impl RussianAccent {
    pub fn new() -> Self {
        RussianAccent {
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

    pub fn contains_word(&self, word: &str) -> bool {
        self.words.iter().any(|w| w == word)
    }

    pub fn count_words(&self) -> usize {
        self.words.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_russian_accent() {
        let mut accent = RussianAccent::new();
        assert_eq!(accent.count_words(), 0);

        accent.add_word(String::from("привет"));
        assert_eq!(accent.count_words(), 1);
        assert!(accent.contains_word("привет"));

        accent.add_word(String::from("мир"));
        assert_eq!(accent.count_words(), 2);
        assert!(accent.contains_word("мир"));

        let removed = accent.remove_word(0);
        assert_eq!(removed, Some(String::from("привет")));
        assert!(!accent.contains_word("привет"));
        assert_eq!(accent.count_words(), 1);

        assert_eq!(accent.get_words(), &vec![String::from("мир")]);
    }
}
