extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn agent_priority_engine_init() {
    // Initialization logic for the module
}

pub extern "C" fn agent_priority_engine_exit() {
    // Cleanup logic for the module
}

pub struct AgentPriorityEngine {
    agents: Vec<String>,
    priorities: Vec<u32>,
}

impl AgentPriorityEngine {
    pub fn new() -> Self {
        AgentPriorityEngine {
            agents: Vec::new(),
            priorities: Vec::new(),
        }
    }

    pub fn add_agent(&mut self, agent_name: &str, priority: u32) {
        self.agents.push(String::from(agent_name));
        self.priorities.push(priority);
    }

    pub fn remove_agent(&mut self, agent_name: &str) -> bool {
        if let Some(index) = self.agents.iter().position(|a| a == agent_name) {
            self.agents.remove(index);
            self.priorities.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_agent_priority(&self, agent_name: &str) -> Option<u32> {
        self.agents.iter().position(|a| a == agent_name).map(|index| self.priorities[index])
    }

    pub fn update_agent_priority(&mut self, agent_name: &str, new_priority: u32) -> bool {
        if let Some(index) = self.agents.iter().position(|a| a == agent_name) {
            self.priorities[index] = new_priority;
            true
        } else {
            false
        }
    }

    pub fn list_agents(&self) -> Vec<&String> {
        self.agents.iter().collect()
    }
}
