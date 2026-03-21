extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AppResultExtractor {
    results: Vec<String>,
}

impl AppResultExtractor {
    pub fn new() -> Self {
        AppResultExtractor {
            results: Vec::new(),
        }
    }

    pub fn add_result(&mut self, result: String) {
        self.results.push(result);
    }

    pub fn get_results(&self) -> &Vec<String> {
        &self.results
    }

    pub fn clear_results(&mut self) {
        self.results.clear();
    }

    pub fn has_results(&self) -> bool {
        !self.results.is_empty()
    }

    pub fn find_result(&self, query: &str) -> Option<&String> {
        self.results.iter().find(|result| result.contains(query))
    }
}
