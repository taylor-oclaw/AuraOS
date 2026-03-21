extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct LangTranslateDocument {
    content: String,
    source_lang: String,
    target_lang: String,
}

impl LangTranslateDocument {
    pub fn new(content: &str, source_lang: &str, target_lang: &str) -> Self {
        LangTranslateDocument {
            content: String::from(content),
            source_lang: String::from(source_lang),
            target_lang: String::from(target_lang),
        }
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn set_content(&mut self, new_content: &str) {
        self.content = String::from(new_content);
    }

    pub fn get_source_lang(&self) -> &str {
        &self.source_lang
    }

    pub fn set_source_lang(&mut self, new_source_lang: &str) {
        self.source_lang = String::from(new_source_lang);
    }

    pub fn get_target_lang(&self) -> &str {
        &self.target_lang
    }

    pub fn set_target_lang(&mut self, new_target_lang: &str) {
        self.target_lang = String::from(new_target_lang);
    }
}
