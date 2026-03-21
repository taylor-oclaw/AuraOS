extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SpeechSignLanguageISL {
    // Example fields for the module
    translations: Vec<(String, String)>,
}

impl SpeechSignLanguageISL {
    pub fn new() -> Self {
        SpeechSignLanguageISL {
            translations: Vec::new(),
        }
    }

    pub fn add_translation(&mut self, speech: &str, sign_language: &str) {
        let speech = String::from(speech);
        let sign_language = String::from(sign_language);
        self.translations.push((speech, sign_language));
    }

    pub fn get_sign_language(&self, speech: &str) -> Option<&String> {
        for (s, sl) in &self.translations {
            if s == speech {
                return Some(sl);
            }
        }
        None
    }

    pub fn remove_translation(&mut self, speech: &str) {
        self.translations.retain(|(s, _)| s != speech);
    }

    pub fn list_translations(&self) -> Vec<&String> {
        self.translations.iter().map(|(_, sl)| sl).collect()
    }
}
