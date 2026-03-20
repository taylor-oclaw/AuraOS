extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut load_balancer = AgentLoadBalancer::new();
    load_balancer.add_agent(String::from("agent1"));
    load_balancer.add_agent(String::from("agent2"));
    load_balancer.add_task(String::from("task1"));
    load_balancer.assign_task_to_agent("task1", "agent1");
    let assigned_agent = load_balancer.get_assigned_agent("task1");
    println!("Task1 is assigned to: {}", assigned_agent.unwrap_or_else(|| String::from("No agent")));
}

pub struct AgentLoadBalancer {
    agents: Vec<String>,
    tasks: Vec<String>,
    assignments: Vec<(String, String)>, // (task, agent)
}

impl AgentLoadBalancer {
    pub fn new() -> Self {
        AgentLoadBalancer {
            agents: Vec::new(),
            tasks: Vec::new(),
            assignments: Vec::new(),
        }
    }

    pub fn add_agent(&mut self, agent_name: String) {
        if !self.agents.contains(&agent_name) {
            self.agents.push(agent_name);
        }
    }

    pub fn remove_agent(&mut self, agent_name: &str) {
        self.agents.retain(|a| a != agent_name);
        self.assignments.retain(|(_, a)| a != agent_name);
    }

    pub fn add_task(&mut self, task_name: String) {
        if !self.tasks.contains(&task_name) {
            self.tasks.push(task_name);
        }
    }

    pub fn remove_task(&mut self, task_name: &str) {
        self.tasks.retain(|t| t != task_name);
        self.assignments.retain(|(t, _)| t != task_name);
    }

    pub fn assign_task_to_agent(&mut self, task_name: &str, agent_name: &str) -> bool {
        if self.agents.contains(&agent_name.to_string()) && self.tasks.contains(&task_name.to_string()) {
            self.assignments.push((task_name.to_string(), agent_name.to_string()));
            true
        } else {
            false
        }
    }

    pub fn get_assigned_agent(&self, task_name: &str) -> Option<String> {
        for (t, a) in &self.assignments {
            if t == task_name {
                return Some(a.clone());
            }
        }
        None
    }

    pub fn list_agents(&self) -> Vec<String> {
        self.agents.clone()
    }

    pub fn list_tasks(&self) -> Vec<String> {
        self.tasks.clone()
    }

    pub fn list_assignments(&self) -> Vec<(String, String)> {
        self.assignments.clone()
    }
}
