extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn agent_cep_engine_init() {
    // Initialization logic for the module
}

pub extern "C" fn agent_cep_engine_exit() {
    // Cleanup logic for the module
}

pub struct AgentCEPEngine {
    rules: Vec<String>,
    events: Vec<String>,
}

impl AgentCEPEngine {
    pub fn new() -> Self {
        AgentCEPEngine {
            rules: Vec::new(),
            events: Vec::new(),
        }
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

    pub fn add_event(&mut self, event: String) {
        self.events.push(event);
    }

    pub fn remove_event(&mut self, index: usize) -> Option<String> {
        if index < self.events.len() {
            Some(self.events.remove(index))
        } else {
            None
        }
    }

    pub fn process_events(&self) -> Vec<String> {
        let mut results = Vec::new();
        for event in &self.events {
            for rule in &self.rules {
                if event.contains(rule) {
                    results.push(event.clone());
                }
            }
        }
        results
    }
}
