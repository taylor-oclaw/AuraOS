extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct AgentTrustChain {
    agents: Vec<String>,
    trust_levels: Vec<u8>,
}

impl AgentTrustChain {
    pub fn new() -> Self {
        AgentTrustChain {
            agents: Vec::new(),
            trust_levels: Vec::new(),
        }
    }

    pub fn add_agent(&mut self, agent_name: &str, trust_level: u8) {
        if trust_level > 100 {
            return; // Trust level should be between 0 and 100
        }
        self.agents.push(String::from(agent_name));
        self.trust_levels.push(trust_level);
    }

    pub fn get_trust_level(&self, agent_name: &str) -> Option<u8> {
        for (i, agent) in self.agents.iter().enumerate() {
            if agent == agent_name {
                return Some(self.trust_levels[i]);
            }
        }
        None
    }

    pub fn update_trust_level(&mut self, agent_name: &str, new_trust_level: u8) {
        if new_trust_level > 100 {
            return; // Trust level should be between 0 and 100
        }
        for (i, agent) in self.agents.iter().enumerate() {
            if agent == agent_name {
                self.trust_levels[i] = new_trust_level;
                break;
            }
        }
    }

    pub fn remove_agent(&mut self, agent_name: &str) {
        let pos = self.agents.iter().position(|agent| agent == agent_name);
        if let Some(index) = pos {
            self.agents.remove(index);
            self.trust_levels.remove(index);
        }
    }

    pub fn list_agents(&self) -> Vec<(String, u8)> {
        self.agents
            .iter()
            .zip(self.trust_levels.iter())
            .map(|(agent, &trust_level)| (String::from(agent), trust_level))
            .collect()
    }
}
