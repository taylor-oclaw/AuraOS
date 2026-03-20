extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentContextManager {
    contexts: Vec<String>,
}

impl AgentContextManager {
    pub fn new() -> Self {
        AgentContextManager {
            contexts: Vec::new(),
        }
    }

    pub fn add_context(&mut self, context: String) {
        if !self.contexts.contains(&context) {
            self.contexts.push(context);
        }
    }

    pub fn remove_context(&mut self, context: &str) -> bool {
        let index = self.contexts.iter().position(|c| c == context);
        match index {
            Some(i) => {
                self.contexts.remove(i);
                true
            }
            None => false,
        }
    }

    pub fn has_context(&self, context: &str) -> bool {
        self.contexts.contains(&String::from(context))
    }

    pub fn list_contexts(&self) -> Vec<String> {
        self.contexts.clone()
    }

    pub fn clear_contexts(&mut self) {
        self.contexts.clear();
    }
}
