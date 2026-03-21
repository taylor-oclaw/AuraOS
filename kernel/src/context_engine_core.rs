extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ContextEngineCore {
    contexts: Vec<String>,
}

impl ContextEngineCore {
    pub fn new() -> Self {
        ContextEngineCore {
            contexts: Vec::new(),
        }
    }

    pub fn add_context(&mut self, context: String) {
        self.contexts.push(context);
    }

    pub fn remove_context(&mut self, index: usize) -> Option<String> {
        if index < self.contexts.len() {
            Some(self.contexts.remove(index))
        } else {
            None
        }
    }

    pub fn get_context(&self, index: usize) -> Option<&String> {
        self.contexts.get(index)
    }

    pub fn list_contexts(&self) -> &[String] {
        &self.contexts
    }

    pub fn clear_contexts(&mut self) {
        self.contexts.clear();
    }
}
