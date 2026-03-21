extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_meeting_multilingual_init() {
    // Initialization code for the module
}

#[no_mangle]
pub extern "C" fn lang_meeting_multilingual_exit() {
    // Cleanup code for the module
}

pub struct MultilingualText {
    translations: Vec<(String, String)>,
}

impl MultilingualText {
    pub fn new() -> Self {
        MultilingualText {
            translations: Vec::new(),
        }
    }

    pub fn add_translation(&mut self, language: &str, text: &str) {
        let lang = String::from(language);
        let txt = String::from(text);
        self.translations.push((lang, txt));
    }

    pub fn get_translation(&self, language: &str) -> Option<&String> {
        for (lang, text) in &self.translations {
            if lang == language {
                return Some(text);
            }
        }
        None
    }

    pub fn remove_translation(&mut self, language: &str) {
        self.translations.retain(|(lang, _)| lang != language);
    }

    pub fn list_languages(&self) -> Vec<String> {
        let mut languages = Vec::new();
        for (lang, _) in &self.translations {
            languages.push(lang.clone());
        }
        languages
    }

    pub fn count_translations(&self) -> usize {
        self.translations.len()
    }
}
