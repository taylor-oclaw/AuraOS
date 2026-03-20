extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentConsensus {
    agents: Vec<String>,
    consensus_reached: bool,
}

impl AgentConsensus {
    pub fn new() -> Self {
        AgentConsensus {
            agents: Vec::new(),
            consensus_reached: false,
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

    pub fn get_agents(&self) -> &[String] {
        &self.agents
    }

    pub fn check_consensus(&mut self, decision: &str) -> bool {
        if self.agents.is_empty() {
            return false;
        }

        // Simple consensus logic: all agents must agree on the same decision
        let first_agent_decision = self.agents[0].clone();
        for agent in &self.agents {
            if agent != &first_agent_decision {
                self.consensus_reached = false;
                return false;
            }
        }

        self.consensus_reached = true;
        true
    }

    pub fn is_consensus_reached(&self) -> bool {
        self.consensus_reached
    }
}
