extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_accent_japanese_init() {
    // Initialization code for the module
}

#[no_mangle]
pub extern "C" fn lang_accent_japanese_exit() {
    // Cleanup code for the module
}

pub struct JapaneseAccent {
    words: Vec<String>,
}

impl JapaneseAccent {
    pub fn new() -> Self {
        JapaneseAccent {
            words: Vec::new(),
        }
    }

    pub fn add_word(&mut self, word: &str) {
        self.words.push(word.to_string());
    }

    pub fn get_words(&self) -> &[String] {
        &self.words
    }

    pub fn count_words(&self) -> usize {
        self.words.len()
    }

    pub fn contains_word(&self, word: &str) -> bool {
        self.words.contains(&word.to_string())
    }

    pub fn remove_word(&mut self, word: &str) {
        if let Some(index) = self.words.iter().position(|w| w == word) {
            self.words.remove(index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_japanese_accent() {
        let mut accent = JapaneseAccent::new();
        assert_eq!(accent.count_words(), 0);

        accent.add_word("こんにちは");
        assert_eq!(accent.count_words(), 1);
        assert!(accent.contains_word("こんにちは"));
        assert!(!accent.contains_word("さようなら"));

        accent.add_word("さようなら");
        assert_eq!(accent.count_words(), 2);
        assert!(accent.contains_word("さようなら"));

        accent.remove_word("こんにちは");
        assert_eq!(accent.count_words(), 1);
        assert!(!accent.contains_word("こんにちは"));
    }
}
