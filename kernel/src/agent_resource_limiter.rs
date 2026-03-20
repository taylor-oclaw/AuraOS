extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentResourceLimiter {
    max_cpu_usage: u32,
    max_memory_usage: u64,
    current_cpu_usage: u32,
    current_memory_usage: u64,
    allowed_agents: Vec<String>,
}

impl AgentResourceLimiter {
    pub fn new(max_cpu_usage: u32, max_memory_usage: u64) -> Self {
        AgentResourceLimiter {
            max_cpu_usage,
            max_memory_usage,
            current_cpu_usage: 0,
            current_memory_usage: 0,
            allowed_agents: Vec::new(),
        }
    }

    pub fn add_allowed_agent(&mut self, agent_name: String) {
        if !self.allowed_agents.contains(&agent_name) {
            self.allowed_agents.push(agent_name);
        }
    }

    pub fn remove_allowed_agent(&mut self, agent_name: &str) {
        self.allowed_agents.retain(|name| name != agent_name);
    }

    pub fn is_agent_allowed(&self, agent_name: &str) -> bool {
        self.allowed_agents.contains(&String::from(agent_name))
    }

    pub fn allocate_resources(&mut self, cpu_usage: u32, memory_usage: u64) -> bool {
        if self.current_cpu_usage + cpu_usage <= self.max_cpu_usage
            && self.current_memory_usage + memory_usage <= self.max_memory_usage
        {
            self.current_cpu_usage += cpu_usage;
            self.current_memory_usage += memory_usage;
            true
        } else {
            false
        }
    }

    pub fn release_resources(&mut self, cpu_usage: u32, memory_usage: u64) {
        if self.current_cpu_usage >= cpu_usage && self.current_memory_usage >= memory_usage {
            self.current_cpu_usage -= cpu_usage;
            self.current_memory_usage -= memory_usage;
        }
    }
}
