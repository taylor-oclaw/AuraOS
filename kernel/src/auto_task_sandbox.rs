extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AutoTaskSandbox {
    tasks: Vec<String>,
}

impl AutoTaskSandbox {
    pub fn new() -> Self {
        AutoTaskSandbox { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task_name: &str) {
        self.tasks.push(String::from(task_name));
    }

    pub fn remove_task(&mut self, task_name: &str) {
        if let Some(index) = self.tasks.iter().position(|t| t == task_name) {
            self.tasks.remove(index);
        }
    }

    pub fn list_tasks(&self) -> Vec<String> {
        self.tasks.clone()
    }

    pub fn has_task(&self, task_name: &str) -> bool {
        self.tasks.contains(&String::from(task_name))
    }

    pub fn clear_tasks(&mut self) {
        self.tasks.clear();
    }
}
