extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct LangTranslateSlangHandler {
    // Example data structure to hold translations or slang mappings
    translations: Vec<(String, String)>,
}

impl LangTranslateSlangHandler {
    pub fn new() -> Self {
        LangTranslateSlangHandler {
            translations: Vec::new(),
        }
    }

    pub fn add_translation(&mut self, from: &str, to: &str) {
        self.translations.push((String::from(from), String::from(to)));
    }

    pub fn translate(&self, text: &str) -> Option<String> {
        for (from, to) in &self.translations {
            if from == text {
                return Some(to.clone());
            }
        }
        None
    }

    pub fn remove_translation(&mut self, from: &str) {
        self.translations.retain(|(f, _)| f != from);
    }

    pub fn list_translations(&self) -> Vec<(String, String)> {
        self.translations.clone()
    }

    pub fn clear_translations(&mut self) {
        self.translations.clear();
    }
}
