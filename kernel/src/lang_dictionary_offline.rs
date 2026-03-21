extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn lang_dictionary_offline_init() {
    // Initialization logic for the module
}

pub extern "C" fn lang_dictionary_offline_exit() {
    // Cleanup logic for the module
}

pub struct LangDictionaryOffline {
    words: Vec<String>,
}

impl LangDictionaryOffline {
    pub fn new() -> Self {
        LangDictionaryOffline { words: Vec::new() }
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

    pub fn contains_word(&self, word: &str) -> bool {
        self.words.contains(&String::from(word))
    }

    pub fn get_all_words(&self) -> Vec<String> {
        self.words.clone()
    }

    pub fn count_words(&self) -> usize {
        self.words.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_word() {
        let mut dict = LangDictionaryOffline::new();
        dict.add_word(String::from("hello"));
        assert_eq!(dict.count_words(), 1);
    }

    #[test]
    fn test_remove_word() {
        let mut dict = LangDictionaryOffline::new();
        dict.add_word(String::from("world"));
        assert!(dict.remove_word("world"));
        assert_eq!(dict.count_words(), 0);
    }

    #[test]
    fn test_contains_word() {
        let mut dict = LangDictionaryOffline::new();
        dict.add_word(String::from("rust"));
        assert!(dict.contains_word("rust"));
    }

    #[test]
    fn test_get_all_words() {
        let mut dict = LangDictionaryOffline::new();
        dict.add_word(String::from("AI"));
        dict.add_word(String::from("kernel"));
        let words = dict.get_all_words();
        assert_eq!(words.len(), 2);
        assert!(words.contains(&String::from("AI")));
        assert!(words.contains(&String::from("kernel")));
    }

    #[test]
    fn test_count_words() {
        let mut dict = LangDictionaryOffline::new();
        dict.add_word(String::from("offline"));
        dict.add_word(String::from("dictionary"));
        assert_eq!(dict.count_words(), 2);
    }
}
