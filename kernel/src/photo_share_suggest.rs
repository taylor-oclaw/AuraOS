extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct PhotoShareSuggest {
    user_id: u32,
    suggestions: Vec<String>,
}

impl PhotoShareSuggest {
    pub fn new(user_id: u32) -> Self {
        PhotoShareSuggest {
            user_id,
            suggestions: Vec::new(),
        }
    }

    pub fn add_suggestion(&mut self, suggestion: String) {
        self.suggestions.push(suggestion);
    }

    pub fn get_suggestions(&self) -> &Vec<String> {
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

    pub fn has_suggestions(&self) -> bool {
        !self.suggestions.is_empty()
    }
}
