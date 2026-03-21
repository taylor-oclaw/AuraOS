extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AuraAssistantFileSuggest {
    suggestions: Vec<String>,
}

impl AuraAssistantFileSuggest {
    pub fn new() -> Self {
        AuraAssistantFileSuggest {
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

    pub fn get_suggestion(&self, index: usize) -> Option<&String> {
        self.suggestions.get(index)
    }

    pub fn list_suggestions(&self) -> &[String] {
        &self.suggestions
    }

    pub fn clear_suggestions(&mut self) {
        self.suggestions.clear();
    }
}
