extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn lang_translate_realtime_init() {
    // Initialization logic for the module
}

pub extern "C" fn lang_translate_realtime_exit() {
    // Cleanup logic for the module
}

pub struct LangTranslateRealtime {
    translations: Vec<(String, String)>,
}

impl LangTranslateRealtime {
    pub fn new() -> Self {
        LangTranslateRealtime {
            translations: Vec::new(),
        }
    }

    pub fn add_translation(&mut self, from: &str, to: &str) {
        let from_str = String::from(from);
        let to_str = String::from(to);
        self.translations.push((from_str, to_str));
    }

    pub fn translate(&self, text: &str) -> Option<&String> {
        for (from, to) in &self.translations {
            if from == text {
                return Some(to);
            }
        }
        None
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

    pub fn list_translations(&self) -> Vec<(&String, &String)> {
        self.translations.iter().map(|(from, to)| (from, to)).collect()
    }
}
