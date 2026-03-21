extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn lang_accent_english_in_init() {
    // Initialization code for the module
}

pub extern "C" fn lang_accent_english_in_exit() {
    // Cleanup code for the module
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

    pub fn get_words(&self) -> &Vec<String> {
        &self.words
    }

    pub fn remove_word(&mut self, word: &str) -> bool {
        if let Some(index) = self.words.iter().position(|w| w == word) {
            self.words.remove(index);
            true
        } else {
            false
        }
    }

    pub fn contains_word(&self, word: &str) -> bool {
        self.words.contains(&String::from(word))
    }

    pub fn count_words(&self) -> usize {
        self.words.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_english_accent() {
        let mut accent = EnglishAccent::new();
        assert_eq!(accent.count_words(), 0);

        accent.add_word(String::from("hello"));
        assert_eq!(accent.count_words(), 1);
        assert!(accent.contains_word("hello"));

        accent.add_word(String::from("world"));
        assert_eq!(accent.count_words(), 2);
        assert!(accent.contains_word("world"));

        assert!(accent.remove_word("hello"));
        assert_eq!(accent.count_words(), 1);
        assert!(!accent.contains_word("hello"));

        assert!(!accent.remove_word("foo"));
        assert_eq!(accent.count_words(), 1);

        let words = accent.get_words();
        assert_eq!(words.len(), 1);
        assert_eq!(words[0], "world");
    }
}
