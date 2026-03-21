extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct RelationshipReconnectSuggest {
    user_id: u32,
    suggestions: Vec<String>,
}

impl RelationshipReconnectSuggest {
    pub fn new(user_id: u32) -> Self {
        RelationshipReconnectSuggest {
            user_id,
            suggestions: Vec::new(),
        }
    }

    pub fn add_suggestion(&mut self, suggestion: String) {
        self.suggestions.push(suggestion);
    }

    pub fn get_user_id(&self) -> u32 {
        self.user_id
    }

    pub fn get_suggestions(&self) -> &Vec<String> {
        &self.suggestions
    }

    pub fn clear_suggestions(&mut self) {
        self.suggestions.clear();
    }

    pub fn has_suggestions(&self) -> bool {
        !self.suggestions.is_empty()
    }
}
