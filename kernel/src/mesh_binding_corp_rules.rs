extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshBindingCorpRules {
    rules: Vec<String>,
}

impl MeshBindingCorpRules {
    pub fn new() -> Self {
        MeshBindingCorpRules { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: String) {
        self.rules.push(rule);
    }

    pub fn remove_rule(&mut self, index: usize) -> Option<String> {
        if index < self.rules.len() {
            Some(self.rules.remove(index))
        } else {
            None
        }
    }

    pub fn get_rule(&self, index: usize) -> Option<&String> {
        self.rules.get(index)
    }

    pub fn list_rules(&self) -> &[String] {
        &self.rules
    }

    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }
}
