extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct PeopleCongratsSuggest {
    names: Vec<String>,
}

impl PeopleCongratsSuggest {
    pub fn new() -> Self {
        PeopleCongratsSuggest {
            names: Vec::new(),
        }
    }

    pub fn add_name(&mut self, name: String) {
        self.names.push(name);
    }

    pub fn remove_name(&mut self, name: &str) -> bool {
        if let Some(index) = self.names.iter().position(|n| n == name) {
            self.names.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_names(&self) -> &[String] {
        &self.names
    }

    pub fn suggest_congratulation(&self, event: &str) -> Option<String> {
        if let Some(name) = self.names.first() {
            Some(format!("Congratulations {} on your {}", name, event))
        } else {
            None
        }
    }

    pub fn clear_names(&mut self) {
        self.names.clear();
    }
}
