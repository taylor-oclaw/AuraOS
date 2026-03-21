extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn lang_accent_english_uk_init() {
    // Initialization code for the module
}

pub extern "C" fn lang_accent_english_uk_exit() {
    // Cleanup code for the module
}

pub struct EnglishUKAccent {
    vocabulary: Vec<String>,
    grammar_rules: Vec<String>,
}

impl EnglishUKAccent {
    pub fn new() -> Self {
        EnglishUKAccent {
            vocabulary: vec![
                String::from("colour"),
                String::from("favourite"),
                String::from("centre"),
                String::from("organisation"),
                String::from("realise"),
            ],
            grammar_rules: vec![
                String::from("Use 'colour' instead of 'color'."),
                String::from("Use 'favourite' instead of 'favorite'."),
                String::from("Use 'centre' instead of 'center'."),
                String::from("Use 'organisation' instead of 'organization'."),
                String::from("Use 'realise' instead of 'realize'."),
            ],
        }
    }

    pub fn get_vocabulary(&self) -> &Vec<String> {
        &self.vocabulary
    }

    pub fn add_word(&mut self, word: String) {
        if !self.vocabulary.contains(&word) {
            self.vocabulary.push(word);
        }
    }

    pub fn remove_word(&mut self, word: &str) {
        if let Some(index) = self.vocabulary.iter().position(|w| w == word) {
            self.vocabulary.remove(index);
        }
    }

    pub fn get_grammar_rules(&self) -> &Vec<String> {
        &self.grammar_rules
    }

    pub fn add_grammar_rule(&mut self, rule: String) {
        if !self.grammar_rules.contains(&rule) {
            self.grammar_rules.push(rule);
        }
    }

    pub fn remove_grammar_rule(&mut self, rule: &str) {
        if let Some(index) = self.grammar_rules.iter().position(|r| r == rule) {
            self.grammar_rules.remove(index);
        }
    }
}
