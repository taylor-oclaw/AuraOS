extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct A2aProtocol {
    agents: Vec<AgentCard>,
    tasks: Vec<Task>,
}

struct AgentCard {
    name: String,
    capabilities: Vec<String>,
    endpoint: String,
}

struct Task {
    id: usize,
    agent: String,
    status: String,
    result: Option<String>,
}

impl A2aProtocol {
    pub fn new() -> Self {
        A2aProtocol { agents: Vec::new(), tasks: Vec::new() }
    }

    pub fn register_agent(&mut self, name: &str, endpoint: &str) {
        self.agents.push(AgentCard {
            name: String::from(name),
            capabilities: Vec::new(),
            endpoint: String::from(endpoint),
        });
    }

    pub fn add_capability(&mut self, agent_name: &str, capability: &str) {
        for agent in self.agents.iter_mut() {
            if agent.name == agent_name {
                agent.capabilities.push(String::from(capability));
                return;
            }
        }
    }

    pub fn create_task(&mut self, agent: &str) -> usize {
        let id = self.tasks.len();
        self.tasks.push(Task {
            id,
            agent: String::from(agent),
            status: String::from("pending"),
            result: None,
        });
        id
    }

    pub fn complete_task(&mut self, id: usize, result: &str) {
        if let Some(task) = self.tasks.get_mut(id) {
            task.status = String::from("completed");
            task.result = Some(String::from(result));
        }
    }

    pub fn agent_count(&self) -> usize {
        self.agents.len()
    }

    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }
}
