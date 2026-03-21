extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct LangTranslateEngine {
    // Placeholder for internal state or data structures
    translations: Vec<(String, String)>,
}

impl LangTranslateEngine {
    pub fn new() -> Self {
        LangTranslateEngine {
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
