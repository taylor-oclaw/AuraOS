extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct BizHandoffKnowledge {
    knowledge_base: Vec<String>,
}

impl BizHandoffKnowledge {
    pub fn new() -> Self {
        BizHandoffKnowledge {
            knowledge_base: Vec::new(),
        }
    }

    pub fn add_knowledge(&mut self, knowledge: String) {
        self.knowledge_base.push(knowledge);
    }

    pub fn remove_knowledge(&mut self, index: usize) -> Option<String> {
        if index < self.knowledge_base.len() {
            Some(self.knowledge_base.remove(index))
        } else {
            None
        }
    }

    pub fn get_knowledge(&self, index: usize) -> Option<&String> {
        self.knowledge_base.get(index)
    }

    pub fn list_knowledge(&self) -> &[String] {
        &self.knowledge_base
    }

    pub fn clear_knowledge(&mut self) {
        self.knowledge_base.clear();
    }
}
