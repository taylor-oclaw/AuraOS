extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct DefsDedupV2 {
    definitions: Vec<String>,
}

impl DefsDedupV2 {
    pub fn new() -> Self {
        DefsDedupV2 {
            definitions: Vec::new(),
        }
    }

    pub fn add_definition(&mut self, definition: String) {
        if !self.definitions.contains(&definition) {
            self.definitions.push(definition);
        }
    }

    pub fn remove_definition(&mut self, definition: &str) -> bool {
        let index = self.definitions.iter().position(|d| d == definition);
        if let Some(i) = index {
            self.definitions.remove(i);
            true
        } else {
            false
        }
    }

    pub fn get_definitions(&self) -> &[String] {
        &self.definitions
    }

    pub fn contains_definition(&self, definition: &str) -> bool {
        self.definitions.contains(&definition.to_string())
    }

    pub fn clear_definitions(&mut self) {
        self.definitions.clear();
    }
}
