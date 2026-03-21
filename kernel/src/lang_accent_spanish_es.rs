extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn lang_accent_spanish_es_init() {
    // Initialization code for the module
}

pub extern "C" fn lang_accent_spanish_es_exit() {
    // Cleanup code for the module
}

pub struct SpanishAccent {
    words: Vec<String>,
}

impl SpanishAccent {
    pub fn new() -> Self {
        SpanishAccent {
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

    pub fn contains_word(&self, word: &str) -> bool {
        self.words.iter().any(|w| w == word)
    }

    pub fn count_words(&self) -> usize {
        self.words.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spanish_accent() {
        let mut accent = SpanishAccent::new();
        assert_eq!(accent.count_words(), 0);

        accent.add_word(String::from("hola"));
        accent.add_word(String::from("mundo"));

        assert_eq!(accent.count_words(), 2);
        assert!(accent.contains_word("hola"));
        assert!(!accent.contains_word("adios"));

        let removed = accent.remove_word(0);
        assert_eq!(removed, Some(String::from("hola")));
        assert_eq!(accent.count_words(), 1);

        let words = accent.get_words();
        assert_eq!(words[0], "mundo");
    }
}
