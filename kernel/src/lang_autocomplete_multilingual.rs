extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct LangAutocompleteMultilingual {
    languages: Vec<String>,
    current_language: usize,
}

impl LangAutocompleteMultilingual {
    pub fn new(languages: Vec<&str>) -> Self {
        let mut lang_vec = Vec::new();
        for lang in languages {
            lang_vec.push(String::from(lang));
        }
        LangAutocompleteMultilingual {
            languages: lang_vec,
            current_language: 0,
        }
    }

    pub fn add_language(&mut self, language: &str) {
        self.languages.push(String::from(language));
    }

    pub fn remove_language(&mut self, index: usize) -> Option<String> {
        if index < self.languages.len() {
            Some(self.languages.remove(index))
        } else {
            None
        }
    }

    pub fn set_current_language(&mut self, index: usize) -> bool {
        if index < self.languages.len() {
            self.current_language = index;
            true
        } else {
            false
        }
    }

    pub fn get_current_language(&self) -> Option<&String> {
        self.languages.get(self.current_language)
    }

    pub fn list_languages(&self) -> &Vec<String> {
        &self.languages
    }
}
