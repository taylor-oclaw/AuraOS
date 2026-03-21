extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct EnterprisePromptPolicy {
    rules: Vec<String>,
}

impl EnterprisePromptPolicy {
    pub fn new() -> Self {
        EnterprisePromptPolicy { rules: Vec::new() }
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

    pub fn get_rules(&self) -> &Vec<String> {
        &self.rules
    }

    pub fn check_prompt(&self, prompt: &str) -> bool {
        for rule in &self.rules {
            if prompt.contains(rule) {
                return false;
            }
        }
        true
    }

    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }
}
