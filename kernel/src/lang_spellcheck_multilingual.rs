extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct LangSpellcheckMultilingual {
    languages: Vec<String>,
    dictionary: Vec<Vec<String>>,
}

impl LangSpellcheckMultilingual {
    pub fn new(languages: Vec<&str>) -> Self {
        let mut dict = Vec::new();
        for _ in 0..languages.len() {
            dict.push(Vec::new());
        }
        LangSpellcheckMultilingual {
            languages: languages.into_iter().map(|s| s.to_string()).collect(),
            dictionary: dict,
        }
    }

    pub fn add_language(&mut self, language: &str) -> bool {
        if !self.languages.contains(&language.to_string()) {
            self.languages.push(language.to_string());
            self.dictionary.push(Vec::new());
            true
        } else {
            false
        }
    }

    pub fn remove_language(&mut self, language: &str) -> bool {
        if let Some(index) = self.languages.iter().position(|l| l == language) {
            self.languages.remove(index);
            self.dictionary.remove(index);
            true
        } else {
            false
        }
    }

    pub fn add_word(&mut self, language: &str, word: &str) -> bool {
        if let Some(index) = self.languages.iter().position(|l| l == language) {
            self.dictionary[index].push(word.to_string());
            true
        } else {
            false
        }
    }

    pub fn check_spelling(&self, language: &str, word: &str) -> bool {
        if let Some(index) = self.languages.iter().position(|l| l == language) {
            self.dictionary[index].contains(&word.to_string())
        } else {
            false
        }
    }

    pub fn list_words(&self, language: &str) -> Option<Vec<String>> {
        self.languages.iter().position(|l| l == language).map(|index| self.dictionary[index].clone())
    }
}
