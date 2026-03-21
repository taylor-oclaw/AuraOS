extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct LangTranslateNamePreserve {
    translations: Vec<(String, String)>,
}

impl LangTranslateNamePreserve {
    pub fn new() -> Self {
        LangTranslateNamePreserve {
            translations: Vec::new(),
        }
    }

    pub fn add_translation(&mut self, from: &str, to: &str) {
        let from_str = String::from(from);
        let to_str = String::from(to);
        self.translations.push((from_str, to_str));
    }

    pub fn translate(&self, name: &str) -> Option<String> {
        for (from, to) in &self.translations {
            if from == name {
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
