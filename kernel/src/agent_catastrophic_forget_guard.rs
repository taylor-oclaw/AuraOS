extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentCatastrophicForgetGuard {
    data: Vec<String>,
}

impl AgentCatastrophicForgetGuard {
    pub fn new() -> Self {
        AgentCatastrophicForgetGuard { data: Vec::new() }
    }

    pub fn add_data(&mut self, item: String) {
        self.data.push(item);
    }

    pub fn remove_data(&mut self, index: usize) -> Option<String> {
        if index < self.data.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }

    pub fn get_data(&self, index: usize) -> Option<&String> {
        self.data.get(index)
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn data_count(&self) -> usize {
        self.data.len()
    }
}
