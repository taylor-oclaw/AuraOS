extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_accent_english_ng_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn lang_accent_english_ng_exit() {
    // Cleanup logic for the module
}

pub struct EnglishAccent {
    words: Vec<String>,
}

impl EnglishAccent {
    pub fn new() -> Self {
        EnglishAccent {
            words: Vec::new(),
        }
    }

    pub fn add_word(&mut self, word: String) {
        self.words.push(word);
    }

    pub fn remove_word(&mut self, word: &str) -> bool {
        if let Some(index) = self.words.iter().position(|w| w == word) {
            self.words.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_words(&self) -> &[String] {
        &self.words
    }

    pub fn contains_word(&self, word: &str) -> bool {
        self.words.contains(&word.to_string())
    }

    pub fn clear_words(&mut self) {
        self.words.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_english_accent() {
        let mut accent = EnglishAccent::new();
        assert_eq!(accent.get_words().len(), 0);

        accent.add_word(String::from("hello"));
        accent.add_word(String::from("world"));
        assert_eq!(accent.get_words().len(), 2);
        assert!(accent.contains_word("hello"));
        assert!(!accent.contains_word("goodbye"));

        assert!(accent.remove_word("hello"));
        assert!(!accent.contains_word("hello"));
        assert_eq!(accent.get_words().len(), 1);

        accent.clear_words();
        assert_eq!(accent.get_words().len(), 0);
    }
}
