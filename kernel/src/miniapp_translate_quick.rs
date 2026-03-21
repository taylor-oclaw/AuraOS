extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn miniapp_translate_quick_init() {
    // Initialization logic for the module
}

pub extern "C" fn miniapp_translate_quick_exit() {
    // Cleanup logic for the module
}

pub struct MiniAppTranslateQuick {
    translations: Vec<(String, String)>,
}

impl MiniAppTranslateQuick {
    pub fn new() -> Self {
        MiniAppTranslateQuick {
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

    pub fn remove_translation(&mut self, from: &str) -> bool {
        let pos = self.translations.iter().position(|(f, _)| f == from);
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
}
