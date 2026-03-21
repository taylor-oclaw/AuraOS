extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn parental_multi_parent_init() {
    // Initialization logic for the module
}

pub extern "C" fn parental_multi_parent_exit() {
    // Cleanup logic for the module
}

pub struct ParentalMultiParent {
    parents: Vec<String>,
}

impl ParentalMultiParent {
    pub fn new() -> Self {
        ParentalMultiParent {
            parents: Vec::new(),
        }
    }

    pub fn add_parent(&mut self, parent_name: &str) {
        self.parents.push(String::from(parent_name));
    }

    pub fn remove_parent(&mut self, parent_name: &str) {
        if let Some(index) = self.parents.iter().position(|p| p == parent_name) {
            self.parents.remove(index);
        }
    }

    pub fn get_parents(&self) -> Vec<String> {
        self.parents.clone()
    }

    pub fn has_parent(&self, parent_name: &str) -> bool {
        self.parents.contains(&String::from(parent_name))
    }

    pub fn clear_parents(&mut self) {
        self.parents.clear();
    }
}
