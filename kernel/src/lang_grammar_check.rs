extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn lang_grammar_check_init() {
    // Initialization code for the module
}

pub extern "C" fn lang_grammar_check_exit() {
    // Cleanup code for the module
}

pub struct GrammarChecker {
    rules: Vec<String>,
}

impl GrammarChecker {
    pub fn new() -> Self {
        GrammarChecker {
            rules: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, rule: String) {
        self.rules.push(rule);
    }

    pub fn check_sentence(&self, sentence: &str) -> bool {
        // Simple logic to check if the sentence contains any of the rules
        for rule in &self.rules {
            if sentence.contains(rule) {
                return true;
            }
        }
        false
    }

    pub fn list_rules(&self) -> Vec<String> {
        self.rules.clone()
    }

    pub fn remove_rule(&mut self, rule: &str) {
        self.rules.retain(|r| r != rule);
    }

    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar_checker() {
        let mut checker = GrammarChecker::new();
        checker.add_rule(String::from("error"));
        checker.add_rule(String::from("warning"));

        assert!(checker.check_sentence("This is an error message."));
        assert!(!checker.check_sentence("This is a normal message."));

        let rules = checker.list_rules();
        assert_eq!(rules.len(), 2);

        checker.remove_rule("error");
        assert!(!checker.check_sentence("This is an error message."));

        checker.clear_rules();
        assert_eq!(checker.list_rules().len(), 0);
    }
}
