extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_accent_thai_init() {
    // Initialization code for the module
}

#[no_mangle]
pub extern "C" fn lang_accent_thai_exit() {
    // Cleanup code for the module
}

pub struct ThaiAccent {
    words: Vec<String>,
}

impl ThaiAccent {
    pub fn new() -> Self {
        ThaiAccent {
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
    fn test_thai_accent() {
        let mut accent = ThaiAccent::new();
        assert_eq!(accent.count_words(), 0);

        accent.add_word(String::from("สวัสดี"));
        assert_eq!(accent.count_words(), 1);
        assert!(accent.contains_word("สวัสดี"));

        accent.add_word(String::from("โลก"));
        assert_eq!(accent.count_words(), 2);
        assert!(accent.contains_word("โลก"));

        assert!(!accent.remove_word("ทิศ"));
        assert!(accent.remove_word("สวัสดี"));
        assert_eq!(accent.count_words(), 1);

        let words = accent.get_words();
        assert_eq!(words[0], "โลก");
    }
}
