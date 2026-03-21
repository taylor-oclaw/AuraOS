extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AutoTaskWebSearch {
    search_terms: Vec<String>,
    results: Vec<String>,
}

impl AutoTaskWebSearch {
    pub fn new() -> Self {
        AutoTaskWebSearch {
            search_terms: Vec::new(),
            results: Vec::new(),
        }
    }

    pub fn add_search_term(&mut self, term: String) {
        self.search_terms.push(term);
    }

    pub fn get_search_terms(&self) -> &Vec<String> {
        &self.search_terms
    }

    pub fn perform_search(&mut self) {
        // Simulate a search operation by adding dummy results
        for term in &self.search_terms {
            let result = format!("Result for: {}", term);
            self.results.push(result);
        }
    }

    pub fn get_results(&self) -> &Vec<String> {
        &self.results
    }

    pub fn clear_results(&mut self) {
        self.results.clear();
    }
}
