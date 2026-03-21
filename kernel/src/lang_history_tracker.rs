extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct LangHistoryTracker {
    history: Vec<String>,
}

impl LangHistoryTracker {
    pub fn new() -> Self {
        LangHistoryTracker {
            history: Vec::new(),
        }
    }

    pub fn add_language(&mut self, language: &str) {
        self.history.push(String::from(language));
    }

    pub fn get_history(&self) -> &[String] {
        &self.history
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    pub fn count_languages(&self) -> usize {
        self.history.len()
    }

    pub fn has_language(&self, language: &str) -> bool {
        self.history.contains(&String::from(language))
    }
}
