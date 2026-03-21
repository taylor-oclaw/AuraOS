extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AppUITextExtract {
    text: String,
}

impl AppUITextExtract {
    pub fn new(text: &str) -> Self {
        AppUITextExtract {
            text: String::from(text),
        }
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = String::from(text);
    }

    pub fn append_text(&mut self, additional_text: &str) {
        self.text.push_str(additional_text);
    }

    pub fn extract_words(&self) -> Vec<String> {
        self.text
            .split_whitespace()
            .map(|s| s.to_string())
            .collect()
    }

    pub fn count_characters(&self) -> usize {
        self.text.chars().count()
    }
}
