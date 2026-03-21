extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ToneDraftSuggestRewrite {
    suggestions: Vec<String>,
}

impl ToneDraftSuggestRewrite {
    pub fn new() -> Self {
        ToneDraftSuggestRewrite {
            suggestions: Vec::new(),
        }
    }

    pub fn add_suggestion(&mut self, suggestion: String) {
        self.suggestions.push(suggestion);
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

    pub fn suggest_rewrite(&self, text: &str) -> Option<&String> {
        if self.has_suggestions() {
            Some(&self.suggestions[0])
        } else {
            None
        }
    }
}
