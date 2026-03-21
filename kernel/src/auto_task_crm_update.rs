extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut crm = AutoTaskCRMUpdate::new();
    crm.add_task("task1".into(), "description1".into());
    crm.update_task_description("task1", "updated description1".into());
    crm.complete_task("task1");
    crm.remove_task("task2");
    let tasks = crm.get_all_tasks();
}

pub struct AutoTaskCRMUpdate {
    tasks: Vec<Task>,
}

impl AutoTaskCRMUpdate {
    pub fn new() -> Self {
        AutoTaskCRMUpdate { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, name: String, description: String) {
        let task = Task {
            name,
            description,
            completed: false,
        };
        self.tasks.push(task);
    }

    pub fn update_task_description(&mut self, name: &str, new_description: String) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.name == name) {
            task.description = new_description;
        }
    }

    pub fn complete_task(&mut self, name: &str) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.name == name) {
            task.completed = true;
        }
    }

    pub fn remove_task(&mut self, name: &str) {
        self.tasks.retain(|t| t.name != name);
    }

    pub fn get_all_tasks(&self) -> Vec<String> {
        self.tasks
            .iter()
            .map(|t| format!("Task: {}, Description: {}, Completed: {}", t.name, t.description, t.completed))
            .collect()
    }
}

struct Task {
    name: String,
    description: String,
    completed: bool,
}
