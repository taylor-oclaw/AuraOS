extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct LangTranslateTechnicalTerm {
    terms: Vec<(String, String)>,
}

impl LangTranslateTechnicalTerm {
    pub fn new() -> Self {
        LangTranslateTechnicalTerm { terms: Vec::new() }
    }

    pub fn add_term(&mut self, term: &str, translation: &str) {
        let term = String::from(term);
        let translation = String::from(translation);
        self.terms.push((term, translation));
    }

    pub fn get_translation(&self, term: &str) -> Option<&String> {
        for (t, translation) in &self.terms {
            if t == term {
                return Some(translation);
            }
        }
        None
    }

    pub fn remove_term(&mut self, term: &str) {
        self.terms.retain(|(t, _)| t != term);
    }

    pub fn list_terms(&self) -> Vec<&String> {
        self.terms.iter().map(|(term, _)| term).collect()
    }
}
