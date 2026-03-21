extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct LangTranslateIdiomHandler {
    // Example data structure to hold translations and idioms
    translations: Vec<(String, String)>,
}

impl LangTranslateIdiomHandler {
    pub fn new() -> Self {
        LangTranslateIdiomHandler {
            translations: Vec::new(),
        }
    }

    pub fn add_translation(&mut self, from: &str, to: &str) {
        let from_str = String::from(from);
        let to_str = String::from(to);
        self.translations.push((from_str, to_str));
    }

    pub fn translate(&self, text: &str) -> Option<String> {
        for (from, to) in &self.translations {
            if from == text {
                return Some(to.clone());
            }
        }
        None
    }

    pub fn list_translations(&self) -> Vec<(String, String)> {
        self.translations.clone()
    }

    pub fn remove_translation(&mut self, from: &str) -> bool {
        let pos = self.translations.iter().position(|(f, _)| f == from);
        if let Some(index) = pos {
            self.translations.remove(index);
            true
        } else {
            false
        }
    }

    pub fn clear_translations(&mut self) {
        self.translations.clear();
    }
}
