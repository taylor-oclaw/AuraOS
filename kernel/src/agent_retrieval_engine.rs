extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let engine = AgentRetrievalEngine::new();
    engine.initialize();
    engine.add_agent(String::from("Agent1"), String::from("Task1"));
    engine.add_agent(String::from("Agent2"), String::from("Task2"));
    engine.list_agents();
    if let Some(task) = engine.get_task_for_agent("Agent1") {
        println!("Agent1 is assigned to task: {}", task);
    }
    engine.remove_agent("Agent2");
    engine.list_agents();
}

pub struct AgentRetrievalEngine {
    agents: Vec<(String, String)>,
}

impl AgentRetrievalEngine {
    pub fn new() -> Self {
        AgentRetrievalEngine { agents: Vec::new() }
    }

    pub fn initialize(&mut self) {
        // Initialization logic
        println!("Agent Retrieval Engine initialized.");
    }

    pub fn add_agent(&mut self, name: String, task: String) {
        self.agents.push((name, task));
        println!("Added agent {} with task {}", name, task);
    }

    pub fn remove_agent(&mut self, name: &str) {
        if let Some(index) = self.agents.iter().position(|(agent_name, _)| agent_name == name) {
            let removed_agent = self.agents.remove(index);
            println!("Removed agent {} with task {}", removed_agent.0, removed_agent.1);
        } else {
            println!("Agent {} not found.", name);
        }
    }

    pub fn list_agents(&self) {
        if self.agents.is_empty() {
            println!("No agents available.");
        } else {
            for (name, task) in &self.agents {
                println!("Agent: {}, Task: {}", name, task);
            }
        }
    }

    pub fn get_task_for_agent(&self, name: &str) -> Option<String> {
        self.agents.iter()
            .find(|(agent_name, _)| agent_name == name)
            .map(|(_, task)| task.clone())
    }
}
