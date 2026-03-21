extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct PeopleAskAboutSuggest {
    questions: Vec<String>,
    suggestions: Vec<String>,
}

impl PeopleAskAboutSuggest {
    pub fn new() -> Self {
        PeopleAskAboutSuggest {
            questions: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    pub fn add_question(&mut self, question: String) {
        self.questions.push(question);
    }

    pub fn add_suggestion(&mut self, suggestion: String) {
        self.suggestions.push(suggestion);
    }

    pub fn get_questions(&self) -> &Vec<String> {
        &self.questions
    }

    pub fn get_suggestions(&self) -> &Vec<String> {
        &self.suggestions
    }

    pub fn find_suggestion_for_question(&self, question: &str) -> Option<&String> {
        for q in &self.questions {
            if q == question {
                return self.suggestions.iter().find(|&s| s.contains(question));
            }
        }
        None
    }
}
