extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DefsSearch {
    definitions: Vec<(String, String)>,
}

impl DefsSearch {
    pub fn new() -> Self {
        DefsSearch {
            definitions: Vec::new(),
        }
    }

    pub fn add_definition(&mut self, term: &str, definition: &str) {
        let term = String::from(term);
        let definition = String::from(definition);
        self.definitions.push((term, definition));
    }

    pub fn get_definition(&self, term: &str) -> Option<&String> {
        for (t, def) in &self.definitions {
            if t == term {
                return Some(def);
            }
        }
        None
    }

    pub fn remove_definition(&mut self, term: &str) {
        self.definitions.retain(|(t, _)| t != term);
    }

    pub fn list_terms(&self) -> Vec<&String> {
        self.definitions.iter().map(|(term, _)| term).collect()
    }

    pub fn count_definitions(&self) -> usize {
        self.definitions.len()
    }
}
