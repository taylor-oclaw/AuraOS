extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialize the module if needed
}

pub extern "C" fn rust_ffi_exit() {
    // Clean up the module if needed
}

pub struct AuraAssistantSuggest {
    suggestions: Vec<String>,
}

impl AuraAssistantSuggest {
    pub fn new() -> Self {
        AuraAssistantSuggest {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aura_assistant_suggest() {
        let mut assistant = AuraAssistantSuggest::new();

        assert!(assistant.list_suggestions().is_empty());

        assistant.add_suggestion(String::from("Suggestion 1"));
        assistant.add_suggestion(String::from("Suggestion 2"));

        assert_eq!(assistant.list_suggestions().len(), 2);
        assert_eq!(assistant.get_suggestion(0), Some(&String::from("Suggestion 1")));
        assert_eq!(assistant.get_suggestion(1), Some(&String::from("Suggestion 2")));

        let removed = assistant.remove_suggestion(0);
        assert_eq!(removed, Some(String::from("Suggestion 1")));
        assert_eq!(assistant.list_suggestions().len(), 1);

        assistant.clear_suggestions();
        assert!(assistant.list_suggestions().is_empty());
    }
}
