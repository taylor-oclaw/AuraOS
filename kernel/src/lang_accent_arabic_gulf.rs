extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn lang_accent_arabic_gulf_init() {
    // Initialization logic for the module
}

pub extern "C" fn lang_accent_arabic_gulf_exit() {
    // Cleanup logic for the module
}

pub struct ArabicGulfAccent {
    words: Vec<String>,
}

impl ArabicGulfAccent {
    pub fn new() -> Self {
        ArabicGulfAccent {
            words: Vec::new(),
        }
    }

    pub fn add_word(&mut self, word: &str) {
        self.words.push(String::from(word));
    }

    pub fn get_words(&self) -> &[String] {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arabic_gulf_accent() {
        let mut accent = ArabicGulfAccent::new();
        assert_eq!(accent.count_words(), 0);

        accent.add_word("مرحبا");
        assert_eq!(accent.count_words(), 1);
        assert!(accent.contains_word("مرحبا"));
        assert!(!accent.contains_word("سلام"));

        accent.remove_word("مرحبا");
        assert_eq!(accent.count_words(), 0);
        assert!(!accent.contains_word("مرحبا"));

        accent.add_word("كيف حالك");
        let words = accent.get_words();
        assert_eq!(words.len(), 1);
        assert_eq!(words[0], "كيف حالك");
    }
}
