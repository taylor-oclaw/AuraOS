extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct AgentAlignmentChecker {
    agents: Vec<String>,
    alignment_criteria: String,
}

impl AgentAlignmentChecker {
    pub fn new(criteria: &str) -> Self {
        AgentAlignmentChecker {
            agents: Vec::new(),
            alignment_criteria: String::from(criteria),
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

    pub fn check_alignment(&self, agent_name: &str) -> bool {
        self.agents.contains(&String::from(agent_name))
    }

    pub fn list_agents(&self) -> Vec<String> {
        self.agents.clone()
    }

    pub fn update_criteria(&mut self, new_criteria: &str) {
        self.alignment_criteria = String::from(new_criteria);
    }
}
