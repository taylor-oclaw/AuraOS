extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct LangTranslateEmail {
    email_content: String,
    source_language: String,
    target_language: String,
}

impl LangTranslateEmail {
    pub fn new(email_content: String, source_language: String, target_language: String) -> Self {
        LangTranslateEmail {
            email_content,
            source_language,
            target_language,
        }
    }

    pub fn get_email_content(&self) -> &str {
        &self.email_content
    }

    pub fn set_email_content(&mut self, content: String) {
        self.email_content = content;
    }

    pub fn get_source_language(&self) -> &str {
        &self.source_language
    }

    pub fn set_source_language(&mut self, language: String) {
        self.source_language = language;
    }

    pub fn get_target_language(&self) -> &str {
        &self.target_language
    }

    pub fn set_target_language(&mut self, language: String) {
        self.target_language = language;
    }
}
