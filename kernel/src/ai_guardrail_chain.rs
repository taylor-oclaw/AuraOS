extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AiGuardrailChain {
    chain: Vec<String>,
}

impl AiGuardrailChain {
    pub fn new() -> Self {
        AiGuardrailChain { chain: Vec::new() }
    }

    pub fn add(&mut self, item: String) {
        self.chain.push(item);
    }

    pub fn remove(&mut self, index: usize) -> Option<String> {
        if index < self.chain.len() {
            let removed = self.chain.remove(index);
            Some(removed)
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> Option<&String> {
        if index < self.chain.len() {
            Some(&self.chain[index])
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.chain.len()
    }

    pub fn is_empty(&self) -> bool {
        self.chain.is_empty()
    }
}