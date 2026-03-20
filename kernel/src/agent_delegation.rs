extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct AgentDelegation {
    agents: Vec<String>,
    tasks: Vec<String>,
}

impl AgentDelegation {
    pub fn new() -> Self {
        AgentDelegation {
            agents: Vec::new(),
            tasks: Vec::new(),
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

    pub fn add_task(&mut self, task_description: &str) {
        self.tasks.push(String::from(task_description));
    }

    pub fn remove_task(&mut self, task_description: &str) -> bool {
        if let Some(index) = self.tasks.iter().position(|t| t == task_description) {
            self.tasks.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_agents(&self) -> Vec<String> {
        self.agents.clone()
    }

    pub fn list_tasks(&self) -> Vec<String> {
        self.tasks.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_delegation() {
        let mut ad = AgentDelegation::new();
        ad.add_agent("Agent1");
        ad.add_agent("Agent2");
        assert_eq!(ad.list_agents(), vec![String::from("Agent1"), String::from("Agent2")]);
        assert!(ad.remove_agent("Agent1"));
        assert_eq!(ad.list_agents(), vec![String::from("Agent2")]);

        ad.add_task("Task1");
        ad.add_task("Task2");
        assert_eq!(ad.list_tasks(), vec![String::from("Task1"), String::from("Task2")]);
        assert!(ad.remove_task("Task1"));
        assert_eq!(ad.list_tasks(), vec![String::from("Task2")]);
    }
}
