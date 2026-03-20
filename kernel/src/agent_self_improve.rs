extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct AgentSelfImprove {
    knowledge_base: Vec<String>,
    tasks_completed: usize,
}

impl AgentSelfImprove {
    pub fn new() -> Self {
        AgentSelfImprove {
            knowledge_base: Vec::new(),
            tasks_completed: 0,
        }
    }

    pub fn add_knowledge(&mut self, knowledge: String) {
        self.knowledge_base.push(knowledge);
    }

    pub fn get_knowledge_count(&self) -> usize {
        self.knowledge_base.len()
    }

    pub fn complete_task(&mut self) {
        self.tasks_completed += 1;
    }

    pub fn get_tasks_completed(&self) -> usize {
        self.tasks_completed
    }

    pub fn search_knowledge(&self, query: &str) -> Vec<String> {
        self.knowledge_base
            .iter()
            .filter(|&knowledge| knowledge.contains(query))
            .cloned()
            .collect()
    }
}
