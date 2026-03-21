extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PredictSearchSuggest {
    suggestions: Vec<String>,
}

impl PredictSearchSuggest {
    pub fn new() -> Self {
        PredictSearchSuggest {
            suggestions: Vec::new(),
        }
    }

    pub fn add_suggestion(&mut self, suggestion: String) {
        self.suggestions.push(suggestion);
    }

    pub fn remove_suggestion(&mut self, index: usize) -> Option<String> {
        if index < self.suggestions.len() {
            Some(self.suggestions.remove(index))
        } else {
            None
        }
    }

    pub fn get_suggestions(&self) -> &[String] {
        &self.suggestions
    }

    pub fn find_suggestion(&self, query: &str) -> Option<&String> {
        self.suggestions.iter().find(|s| s.contains(query))
    }

    pub fn clear_suggestions(&mut self) {
        self.suggestions.clear();
    }
}
