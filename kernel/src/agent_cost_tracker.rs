extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

#[derive(Debug)]
pub struct AgentCostTracker {
    agent_name: String,
    costs: Vec<u32>,
    total_cost: u32,
}

impl AgentCostTracker {
    pub fn new(agent_name: &str) -> Self {
        AgentCostTracker {
            agent_name: String::from(agent_name),
            costs: Vec::new(),
            total_cost: 0,
        }
    }

    pub fn add_cost(&mut self, cost: u32) {
        self.costs.push(cost);
        self.total_cost += cost;
    }

    pub fn get_total_cost(&self) -> u32 {
        self.total_cost
    }

    pub fn get_average_cost(&self) -> Option<u32> {
        if self.costs.is_empty() {
            None
        } else {
            Some(self.total_cost / self.costs.len() as u32)
        }
    }

    pub fn list_costs(&self) -> &Vec<u32> {
        &self.costs
    }

    pub fn clear_costs(&mut self) {
        self.costs.clear();
        self.total_cost = 0;
    }
}
