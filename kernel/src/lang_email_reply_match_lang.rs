extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct LangEmailReplyMatcher {
    language: String,
    email_content: String,
}

impl LangEmailReplyMatcher {
    pub fn new(language: &str, email_content: &str) -> Self {
        LangEmailReplyMatcher {
            language: String::from(language),
            email_content: String::from(email_content),
        }
    }

    pub fn get_language(&self) -> &str {
        &self.language
    }

    pub fn set_language(&mut self, language: &str) {
        self.language = String::from(language);
    }

    pub fn get_email_content(&self) -> &str {
        &self.email_content
    }

    pub fn set_email_content(&mut self, email_content: &str) {
        self.email_content = String::from(email_content);
    }

    pub fn is_reply_in_language(&self, reply_content: &str) -> bool {
        // Simple heuristic: check if the language appears in the reply content
        reply_content.contains(&self.language)
    }

    pub fn extract_replies(&self) -> Vec<String> {
        // Split email content by lines and filter out replies (lines starting with ">")
        self.email_content
            .lines()
            .filter(|line| line.trim_start().starts_with('>'))
            .map(String::from)
            .collect()
    }
}
