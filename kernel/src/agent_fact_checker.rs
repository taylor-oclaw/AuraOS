extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentFactChecker {
    facts: Vec<String>,
}

impl AgentFactChecker {
    pub fn new() -> Self {
        AgentFactChecker { facts: Vec::new() }
    }

    pub fn add_fact(&mut self, fact: String) {
        self.facts.push(fact);
    }

    pub fn remove_fact(&mut self, index: usize) -> Option<String> {
        if index < self.facts.len() {
            Some(self.facts.remove(index))
        } else {
            None
        }
    }

    pub fn get_fact(&self, index: usize) -> Option<&String> {
        self.facts.get(index)
    }

    pub fn has_fact(&self, fact: &str) -> bool {
        self.facts.iter().any(|f| f == fact)
    }

    pub fn list_facts(&self) -> &[String] {
        &self.facts
    }
}
