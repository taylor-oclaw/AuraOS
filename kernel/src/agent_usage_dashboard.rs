extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentUsageDashboard {
    agent_name: String,
    usage_data: Vec<u32>,
}

impl AgentUsageDashboard {
    pub fn new(agent_name: &str) -> Self {
        AgentUsageDashboard {
            agent_name: String::from(agent_name),
            usage_data: Vec::new(),
        }
    }

    pub fn add_usage(&mut self, data: u32) {
        self.usage_data.push(data);
    }

    pub fn get_total_usage(&self) -> u32 {
        self.usage_data.iter().sum()
    }

    pub fn get_average_usage(&self) -> Option<u32> {
        if self.usage_data.is_empty() {
            None
        } else {
            Some(self.get_total_usage() / self.usage_data.len() as u32)
        }
    }

    pub fn get_max_usage(&self) -> Option<u32> {
        self.usage_data.iter().max().copied()
    }

    pub fn get_min_usage(&self) -> Option<u32> {
        self.usage_data.iter().min().copied()
    }
}