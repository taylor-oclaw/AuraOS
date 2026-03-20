extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct DefsWatcher {
    definitions: Vec<String>,
}

impl DefsWatcher {
    pub fn new() -> Self {
        DefsWatcher {
            definitions: Vec::new(),
        }
    }

    pub fn add_definition(&mut self, definition: &str) {
        self.definitions.push(String::from(definition));
    }

    pub fn remove_definition(&mut self, index: usize) -> Option<String> {
        if index < self.definitions.len() {
            Some(self.definitions.remove(index))
        } else {
            None
        }
    }

    pub fn get_definitions(&self) -> &Vec<String> {
        &self.definitions
    }

    pub fn find_definition(&self, definition: &str) -> Option<usize> {
        self.definitions.iter().position(|d| d == definition)
    }

    pub fn clear_definitions(&mut self) {
        self.definitions.clear();
    }
}
