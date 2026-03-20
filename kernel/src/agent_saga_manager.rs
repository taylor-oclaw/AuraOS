extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentSagaManager {
    sagas: Vec<String>,
}

impl AgentSagaManager {
    pub fn new() -> Self {
        AgentSagaManager {
            sagas: Vec::new(),
        }
    }

    pub fn add_saga(&mut self, saga_name: &str) {
        self.sagas.push(String::from(saga_name));
    }

    pub fn remove_saga(&mut self, saga_name: &str) -> bool {
        if let Some(index) = self.sagas.iter().position(|s| s == saga_name) {
            self.sagas.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_sagas(&self) -> Vec<String> {
        self.sagas.clone()
    }

    pub fn has_saga(&self, saga_name: &str) -> bool {
        self.sagas.contains(&String::from(saga_name))
    }

    pub fn count_sagas(&self) -> usize {
        self.sagas.len()
    }
}
