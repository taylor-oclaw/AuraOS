extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn lang_accent_arabic_eg_init() {
    // Initialization logic for the module
}

pub extern "C" fn lang_accent_arabic_eg_exit() {
    // Cleanup logic for the module
}

pub struct ArabicAccent {
    words: Vec<String>,
}

impl ArabicAccent {
    pub fn new() -> Self {
        ArabicAccent {
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

    pub fn contains_word(&self, word: &str) -> bool {
        self.words.iter().any(|w| w == word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arabic_accent() {
        let mut accent = ArabicAccent::new();
        assert_eq!(accent.count_words(), 0);

        accent.add_word(String::from("مرحبا"));
        assert_eq!(accent.count_words(), 1);
        assert!(accent.contains_word("مرحبا"));

        accent.add_word(String::from("عالم"));
        assert_eq!(accent.count_words(), 2);
        assert!(accent.contains_word("عالم"));

        let removed = accent.remove_word(0);
        assert_eq!(removed, Some(String::from("مرحبا")));
        assert_eq!(accent.count_words(), 1);

        assert!(!accent.contains_word("مرحبا"));
    }
}
