extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_accent_swahili_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn lang_accent_swahili_exit() {
    // Cleanup logic for the module
}

pub struct SwahiliAccent {
    words: Vec<String>,
}

impl SwahiliAccent {
    pub fn new() -> Self {
        SwahiliAccent {
            words: Vec::new(),
        }
    }

    pub fn add_word(&mut self, word: String) {
        self.words.push(word);
    }

    pub fn get_words(&self) -> &Vec<String> {
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
    fn test_swahili_accent() {
        let mut accent = SwahiliAccent::new();
        assert_eq!(accent.count_words(), 0);

        accent.add_word(String::from("Hujambo"));
        assert_eq!(accent.count_words(), 1);
        assert!(accent.contains_word("Hujambo"));

        accent.add_word(String::from("Nǐ hǎo"));
        assert_eq!(accent.count_words(), 2);
        assert!(accent.contains_word("Nǐ hǎo"));

        accent.remove_word("Hujambo");
        assert_eq!(accent.count_words(), 1);
        assert!(!accent.contains_word("Hujambo"));

        let words = accent.get_words();
        assert_eq!(words.len(), 1);
        assert_eq!(words[0], "Nǐ hǎo");
    }
}
