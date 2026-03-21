extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_tts_multilingual_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn lang_tts_multilingual_exit() {
    // Cleanup logic for the module
}

pub struct LangTTSMultilingual {
    languages: Vec<String>,
    current_language: String,
}

impl LangTTSMultilingual {
    pub fn new(languages: Vec<&str>) -> Self {
        let mut lang_vec = Vec::new();
        for lang in languages {
            lang_vec.push(String::from(lang));
        }
        LangTTSMultilingual {
            languages: lang_vec,
            current_language: String::from("en"), // Default to English
        }
    }

    pub fn add_language(&mut self, language: &str) {
        if !self.languages.contains(&String::from(language)) {
            self.languages.push(String::from(language));
        }
    }

    pub fn remove_language(&mut self, language: &str) {
        self.languages.retain(|l| l != language);
    }

    pub fn set_current_language(&mut self, language: &str) -> bool {
        if self.languages.contains(&String::from(language)) {
            self.current_language = String::from(language);
            true
        } else {
            false
        }
    }

    pub fn get_current_language(&self) -> &str {
        &self.current_language
    }

    pub fn list_languages(&self) -> Vec<&str> {
        self.languages.iter().map(|l| l.as_str()).collect()
    }
}
