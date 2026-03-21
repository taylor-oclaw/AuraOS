extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_accent_german_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn lang_accent_german_exit() {
    // Cleanup logic for the module
}

pub struct GermanAccent {
    words: Vec<String>,
}

impl GermanAccent {
    pub fn new() -> Self {
        GermanAccent {
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

    pub fn get_word_count(&self) -> usize {
        self.words.len()
    }

    pub fn clear_words(&mut self) {
        self.words.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_german_accent() {
        let mut accent = GermanAccent::new();
        assert_eq!(accent.get_word_count(), 0);

        accent.add_word(String::from("hallo"));
        accent.add_word(String::from("welt"));
        assert_eq!(accent.get_word_count(), 2);
        assert_eq!(accent.get_words()[0], "hallo");
        assert_eq!(accent.get_words()[1], "welt");

        let removed = accent.remove_word(0);
        assert_eq!(removed, Some(String::from("hallo")));
        assert_eq!(accent.get_word_count(), 1);

        accent.clear_words();
        assert_eq!(accent.get_word_count(), 0);
    }
}
