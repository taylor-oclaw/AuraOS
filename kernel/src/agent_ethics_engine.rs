extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentEthicsEngine {
    rules: Vec<String>,
}

impl AgentEthicsEngine {
    pub fn new() -> Self {
        AgentEthicsEngine { rules: Vec::new() }
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

    pub fn evaluate_action(&self, action: String) -> bool {
        for rule in &self.rules {
            if self.matches_rule(&action, rule) {
                return false;
            }
        }
        true
    }

    fn matches_rule(&self, action: &str, rule: &str) -> bool {
        // Simple substring match for demonstration purposes
        action.contains(rule)
    }
}
