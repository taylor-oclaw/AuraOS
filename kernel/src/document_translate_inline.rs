extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DocumentTranslator {
    translations: Vec<(String, String)>,
}

impl DocumentTranslator {
    pub fn new() -> Self {
        DocumentTranslator {
            translations: Vec::new(),
        }
    }

    pub fn add_translation(&mut self, original: &str, translation: &str) {
        let original = String::from(original);
        let translation = String::from(translation);
        self.translations.push((original, translation));
    }

    pub fn translate(&self, text: &str) -> Option<String> {
        for (original, translation) in &self.translations {
            if original == text {
                return Some(translation.clone());
            }
        }
        None
    }

    pub fn remove_translation(&mut self, original: &str) -> bool {
        let pos = self.translations.iter().position(|&(ref o, _)| o == original);
        match pos {
            Some(index) => {
                self.translations.remove(index);
                true
            }
            None => false,
        }
    }

    pub fn list_translations(&self) -> Vec<(String, String)> {
        self.translations.clone()
    }

    pub fn clear_translations(&mut self) {
        self.translations.clear();
    }
}
