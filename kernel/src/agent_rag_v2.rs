extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let agent = AgentRagV2::new("AgentRagV2".into(), 1.0);
    agent.greet();
    agent.update_version(2.0);
    agent.add_task("Task1".into());
    agent.add_task("Task2".into());
    agent.list_tasks();
}

pub struct AgentRagV2 {
    name: String,
    version: f32,
    tasks: Vec<String>,
}

impl AgentRagV2 {
    pub fn new(name: String, version: f32) -> Self {
        AgentRagV2 {
            name,
            version,
            tasks: Vec::new(),
        }
    }

    pub fn greet(&self) {
        // Simulate a greeting message
        let message = format!("Hello from {} v{}", self.name, self.version);
        unsafe { println!("{}", message); } // Assuming a safe way to print in kernel context
    }

    pub fn update_version(&mut self, new_version: f32) {
        self.version = new_version;
        println!("Updated version to {}", self.version);
    }

    pub fn add_task(&mut self, task: String) {
        self.tasks.push(task);
        println!("Task added");
    }

    pub fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks available");
        } else {
            for (index, task) in self.tasks.iter().enumerate() {
                println!("Task {}: {}", index + 1, task);
            }
        }
    }

    pub fn clear_tasks(&mut self) {
        self.tasks.clear();
        println!("All tasks cleared");
    }
}
