extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ToneAbbreviationStyle {
    abbreviations: Vec<(String, String)>,
}

impl ToneAbbreviationStyle {
    pub fn new() -> Self {
        ToneAbbreviationStyle {
            abbreviations: Vec::new(),
        }
    }

    pub fn add_abbreviation(&mut self, full_form: &str, abbreviation: &str) {
        let pair = (String::from(full_form), String::from(abbreviation));
        self.abbreviations.push(pair);
    }

    pub fn get_abbreviation(&self, full_form: &str) -> Option<&str> {
        for (full, abbrev) in &self.abbreviations {
            if full == full_form {
                return Some(abbrev);
            }
        }
        None
    }

    pub fn remove_abbreviation(&mut self, full_form: &str) -> bool {
        let pos = self.abbreviations.iter().position(|(full, _)| full == full_form);
        if let Some(index) = pos {
            self.abbreviations.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_abbreviations(&self) -> Vec<(String, String)> {
        self.abbreviations.iter().cloned().collect()
    }

    pub fn count_abbreviations(&self) -> usize {
        self.abbreviations.len()
    }
}
