extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct LanguageDominantLanguage {
    languages: Vec<String>,
}

impl LanguageDominantLanguage {
    pub fn new() -> Self {
        LanguageDominantLanguage {
            languages: Vec::new(),
        }
    }

    pub fn add_language(&mut self, language: String) {
        self.languages.push(language);
    }

    pub fn remove_language(&mut self, language: &str) -> bool {
        if let Some(index) = self.languages.iter().position(|l| l == language) {
            self.languages.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_languages(&self) -> &[String] {
        &self.languages
    }

    pub fn is_language_present(&self, language: &str) -> bool {
        self.languages.contains(&language.to_string())
    }

    pub fn count_languages(&self) -> usize {
        self.languages.len()
    }
}
