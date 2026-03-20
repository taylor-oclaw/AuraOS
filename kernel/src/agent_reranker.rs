extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut reranker = AgentReranker::new();
    reranker.add_agent("Agent1".into());
    reranker.add_agent("Agent2".into());
    reranker.rank_agents();
}

pub struct AgentReranker {
    agents: Vec<String>,
    rankings: Vec<usize>,
}

impl AgentReranker {
    pub fn new() -> Self {
        AgentReranker {
            agents: Vec::new(),
            rankings: Vec::new(),
        }
    }

    pub fn add_agent(&mut self, agent_name: String) {
        self.agents.push(agent_name);
    }

    pub fn remove_agent(&mut self, agent_name: &str) -> bool {
        if let Some(index) = self.agents.iter().position(|a| a == agent_name) {
            self.agents.remove(index);
            self.rankings.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_agents(&self) -> &Vec<String> {
        &self.agents
    }

    pub fn rank_agents(&mut self) {
        // Simple ranking logic: reverse order of addition
        self.rankings = (0..self.agents.len()).rev().collect();
    }

    pub fn get_ranking(&self) -> &Vec<usize> {
        &self.rankings
    }
}
