extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshFirewall {
    rules: Vec<String>,
}

impl MeshFirewall {
    pub fn new() -> Self {
        MeshFirewall { rules: Vec::new() }
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

    pub fn list_rules(&self) -> Vec<String> {
        self.rules.clone()
    }

    pub fn has_rule(&self, rule: &str) -> bool {
        self.rules.iter().any(|r| r == rule)
    }

    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }
}
