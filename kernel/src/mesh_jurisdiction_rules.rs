extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshJurisdictionRules {
    rules: Vec<String>,
}

impl MeshJurisdictionRules {
    pub fn new() -> Self {
        MeshJurisdictionRules { rules: Vec::new() }
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

    pub fn get_rules(&self) -> &[String] {
        &self.rules
    }

    pub fn has_rule(&self, rule: &str) -> bool {
        self.rules.iter().any(|r| r == rule)
    }

    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }
}
