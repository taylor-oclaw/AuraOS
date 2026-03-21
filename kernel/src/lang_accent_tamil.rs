extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_accent_tamil_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn lang_accent_tamil_exit() {
    // Cleanup logic for the module
}

pub struct TamilAccent {
    words: Vec<String>,
}

impl TamilAccent {
    pub fn new() -> Self {
        TamilAccent {
            words: Vec::new(),
        }
    }

    pub fn add_word(&mut self, word: String) {
        self.words.push(word);
    }

    pub fn get_words(&self) -> &Vec<String> {
        &self.words
    }

    pub fn count_words(&self) -> usize {
        self.words.len()
    }

    pub fn contains_word(&self, word: &str) -> bool {
        self.words.iter().any(|w| w == word)
    }

    pub fn remove_word(&mut self, word: &str) {
        if let Some(pos) = self.words.iter().position(|w| w == word) {
            self.words.remove(pos);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tamil_accent() {
        let mut accent = TamilAccent::new();
        assert_eq!(accent.count_words(), 0);

        accent.add_word(String::from("வணக்கம்"));
        assert_eq!(accent.count_words(), 1);
        assert!(accent.contains_word("வணக்கம்"));

        accent.add_word(String::from("நல்லது"));
        assert_eq!(accent.count_words(), 2);
        assert!(accent.contains_word("நல்லது"));

        accent.remove_word("வணக்கம்");
        assert_eq!(accent.count_words(), 1);
        assert!(!accent.contains_word("வணக்கம்"));
    }
}
