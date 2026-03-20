extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AuraAgentStore {
    agents: Vec<String>,
}

impl AuraAgentStore {
    pub fn new() -> Self {
        AuraAgentStore {
            agents: Vec::new(),
        }
    }

    pub fn add_agent(&mut self, agent_name: &str) {
        self.agents.push(String::from(agent_name));
    }

    pub fn remove_agent(&mut self, agent_name: &str) {
        if let Some(index) = self.agents.iter().position(|a| a == agent_name) {
            self.agents.remove(index);
        }
    }

    pub fn get_agents(&self) -> &[String] {
        &self.agents
    }

    pub fn has_agent(&self, agent_name: &str) -> bool {
        self.agents.contains(&String::from(agent_name))
    }

    pub fn clear_agents(&mut self) {
        self.agents.clear();
    }
}
