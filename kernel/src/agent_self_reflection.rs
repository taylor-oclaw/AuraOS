extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    let agent = AgentSelfReflection::new("Agent-123".into(), vec!["Task1".into(), "Task2".into()]);
    agent.add_task("Task3".into());
    agent.remove_task("Task2".into());
    let tasks = agent.get_tasks();
    let status = agent.get_status();
    agent.update_status("Active".into());

    loop {}
}

pub struct AgentSelfReflection {
    id: String,
    tasks: Vec<String>,
    status: String,
}

impl AgentSelfReflection {
    pub fn new(id: String, tasks: Vec<String>) -> Self {
        AgentSelfReflection {
            id,
            tasks,
            status: "Idle".into(),
        }
    }

    pub fn add_task(&mut self, task: String) {
        if !self.tasks.contains(&task) {
            self.tasks.push(task);
        }
    }

    pub fn remove_task(&mut self, task: String) {
        self.tasks.retain(|t| *t != task);
    }

    pub fn get_tasks(&self) -> Vec<String> {
        self.tasks.clone()
    }

    pub fn get_status(&self) -> String {
        self.status.clone()
    }

    pub fn update_status(&mut self, status: String) {
        self.status = status;
    }
}
