extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentKnowledgeBase {
    knowledge: Vec<String>,
}

impl AgentKnowledgeBase {
    pub fn new() -> Self {
        AgentKnowledgeBase {
            knowledge: Vec::new(),
        }
    }

    pub fn add_knowledge(&mut self, fact: &str) {
        self.knowledge.push(String::from(fact));
    }

    pub fn remove_knowledge(&mut self, index: usize) -> Option<String> {
        if index < self.knowledge.len() {
            Some(self.knowledge.remove(index))
        } else {
            None
        }
    }

    pub fn get_knowledge(&self, index: usize) -> Option<&String> {
        self.knowledge.get(index)
    }

    pub fn contains_knowledge(&self, fact: &str) -> bool {
        self.knowledge.iter().any(|k| k == fact)
    }

    pub fn list_knowledge(&self) -> &[String] {
        &self.knowledge
    }
}
