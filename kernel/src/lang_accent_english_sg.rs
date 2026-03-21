extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_accent_english_sg_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn lang_accent_english_sg_exit() {
    // Cleanup logic for the module
}

pub struct EnglishSG {
    words: Vec<String>,
}

impl EnglishSG {
    pub fn new() -> Self {
        EnglishSG {
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

    pub fn count_words(&self) -> usize {
        self.words.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_english_sg() {
        let mut sg = EnglishSG::new();
        assert_eq!(sg.count_words(), 0);

        sg.add_word(String::from("hello"));
        sg.add_word(String::from("world"));
        assert_eq!(sg.count_words(), 2);
        assert!(sg.contains_word("hello"));
        assert!(!sg.contains_word("goodbye"));

        let words = sg.get_words();
        assert_eq!(words.len(), 2);
        assert_eq!(words[0], "hello");
        assert_eq!(words[1], "world");

        assert!(sg.remove_word("hello"));
        assert!(!sg.contains_word("hello"));
        assert_eq!(sg.count_words(), 1);

        assert!(!sg.remove_word("goodbye"));
        assert_eq!(sg.count_words(), 1);
    }
}
