extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_accent_italian_init() {
    // Initialization code for the module
}

#[no_mangle]
pub extern "C" fn lang_accent_italian_exit() {
    // Cleanup code for the module
}

pub struct ItalianAccent {
    words: Vec<String>,
}

impl ItalianAccent {
    pub fn new() -> Self {
        ItalianAccent {
            words: Vec::new(),
        }
    }

    pub fn add_word(&mut self, word: &str) {
        self.words.push(word.to_string());
    }

    pub fn get_words(&self) -> &[String] {
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
        self.words.contains(&word.to_string())
    }

    pub fn count_words(&self) -> usize {
        self.words.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_italian_accent() {
        let mut accent = ItalianAccent::new();
        assert_eq!(accent.count_words(), 0);

        accent.add_word("ciao");
        accent.add_word("mondo");
        assert_eq!(accent.count_words(), 2);
        assert!(accent.contains_word("ciao"));
        assert!(!accent.contains_word("hello"));

        let words = accent.get_words();
        assert_eq!(words.len(), 2);
        assert_eq!(words[0], "ciao");
        assert_eq!(words[1], "mondo");

        assert!(accent.remove_word("ciao"));
        assert!(!accent.contains_word("ciao"));
        assert_eq!(accent.count_words(), 1);

        accent.add_word("ciao");
        assert_eq!(accent.count_words(), 2);
    }
}
