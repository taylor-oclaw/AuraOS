extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn lang_code_switch_response_init() {
    // Initialization code for the module
}

pub extern "C" fn lang_code_switch_response_exit() {
    // Cleanup code for the module
}

pub struct LangCodeSwitchResponse {
    supported_languages: Vec<String>,
    current_language: String,
}

impl LangCodeSwitchResponse {
    pub fn new(languages: Vec<&str>, default_language: &str) -> Self {
        let mut supported_languages = Vec::new();
        for lang in languages {
            supported_languages.push(String::from(lang));
        }
        LangCodeSwitchResponse {
            supported_languages,
            current_language: String::from(default_language),
        }
    }

    pub fn get_current_language(&self) -> &str {
        &self.current_language
    }

    pub fn set_current_language(&mut self, language: &str) -> bool {
        if self.supported_languages.contains(&String::from(language)) {
            self.current_language = String::from(language);
            true
        } else {
            false
        }
    }

    pub fn add_supported_language(&mut self, language: &str) {
        if !self.supported_languages.contains(&String::from(language)) {
            self.supported_languages.push(String::from(language));
        }
    }

    pub fn remove_supported_language(&mut self, language: &str) -> bool {
        let pos = self.supported_languages.iter().position(|x| x == language);
        if let Some(index) = pos {
            self.supported_languages.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_supported_languages(&self) -> Vec<&str> {
        self.supported_languages.iter().map(|s| s.as_str()).collect()
    }
}
