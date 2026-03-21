extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn lang_accent_bengali_init() {
    // Initialization logic for the Bengali language accent module
}

pub extern "C" fn lang_accent_bengali_exit() {
    // Cleanup logic for the Bengali language accent module
}

pub struct BengaliAccent {
    rules: Vec<String>,
}

impl BengaliAccent {
    pub fn new() -> Self {
        BengaliAccent {
            rules: vec![
                String::from("Add a soft 'h' sound at the end of words"),
                String::from("Use double vowels for emphasis"),
                String::from("Pronounce 'r' as 'l' in most cases"),
                String::from("Stress on the first syllable of each word"),
                String::from("Use nasal sounds in certain consonants"),
            ],
        }
    }

    pub fn get_rule(&self, index: usize) -> Option<&str> {
        self.rules.get(index).map(|s| s.as_str())
    }

    pub fn add_rule(&mut self, rule: &str) {
        self.rules.push(String::from(rule));
    }

    pub fn remove_rule(&mut self, index: usize) -> bool {
        if index < self.rules.len() {
            self.rules.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_rules(&self) -> Vec<&str> {
        self.rules.iter().map(|s| s.as_str()).collect()
    }

    pub fn count_rules(&self) -> usize {
        self.rules.len()
    }
}
