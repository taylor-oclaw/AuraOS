extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AuraAssistantReplySuggest {
    suggestions: Vec<String>,
}

impl AuraAssistantReplySuggest {
    pub fn new() -> Self {
        AuraAssistantReplySuggest {
            suggestions: Vec::new(),
        }
    }

    pub fn add_suggestion(&mut self, suggestion: String) {
        self.suggestions.push(suggestion);
    }

    pub fn get_suggestions(&self) -> &[String] {
        &self.suggestions
    }

    pub fn remove_suggestion(&mut self, index: usize) -> Option<String> {
        if index < self.suggestions.len() {
            Some(self.suggestions.remove(index))
        } else {
            None
        }
    }

    pub fn clear_suggestions(&mut self) {
        self.suggestions.clear();
    }

    pub fn find_suggestion(&self, query: &str) -> Option<&String> {
        self.suggestions.iter().find(|s| s.contains(query))
    }
}
