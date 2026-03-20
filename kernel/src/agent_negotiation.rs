extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn init_module() -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn cleanup_module() -> i32 {
    0
}

pub struct AgentNegotiation {
    agents: Vec<String>,
    negotiations: Vec<(String, String)>,
}

impl AgentNegotiation {
    pub fn new() -> Self {
        AgentNegotiation {
            agents: Vec::new(),
            negotiations: Vec::new(),
        }
    }

    pub fn add_agent(&mut self, agent_name: &str) {
        self.agents.push(String::from(agent_name));
    }

    pub fn remove_agent(&mut self, agent_name: &str) -> bool {
        if let Some(index) = self.agents.iter().position(|a| a == agent_name) {
            self.agents.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_agents(&self) -> Vec<String> {
        self.agents.clone()
    }

    pub fn negotiate(&mut self, agent1: &str, agent2: &str) -> bool {
        if self.agents.contains(&String::from(agent1)) && self.agents.contains(&String::from(agent2)) {
            self.negotiations.push((String::from(agent1), String::from(agent2)));
            true
        } else {
            false
        }
    }

    pub fn list_negotiations(&self) -> Vec<(String, String)> {
        self.negotiations.clone()
    }
}
