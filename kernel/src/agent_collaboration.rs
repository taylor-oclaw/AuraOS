extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct AgentCollaboration {
    agents: Vec<String>,
    tasks: Vec<String>,
}

impl AgentCollaboration {
    pub fn new() -> Self {
        AgentCollaboration {
            agents: Vec::new(),
            tasks: Vec::new(),
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

    pub fn add_task(&mut self, task_description: &str) {
        self.tasks.push(String::from(task_description));
    }

    pub fn remove_task(&mut self, task_description: &str) {
        if let Some(index) = self.tasks.iter().position(|t| t == task_description) {
            self.tasks.remove(index);
        }
    }

    pub fn list_agents(&self) -> Vec<String> {
        self.agents.clone()
    }

    pub fn list_tasks(&self) -> Vec<String> {
        self.tasks.clone()
    }
}
