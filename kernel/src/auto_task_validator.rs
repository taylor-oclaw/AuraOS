extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let validator = AutoTaskValidator::new();
    validator.validate_task("example_task");
}

pub struct AutoTaskValidator {
    tasks: Vec<String>,
}

impl AutoTaskValidator {
    pub fn new() -> Self {
        AutoTaskValidator { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task_name: &str) {
        self.tasks.push(String::from(task_name));
    }

    pub fn remove_task(&mut self, task_name: &str) {
        if let Some(index) = self.tasks.iter().position(|t| t == task_name) {
            self.tasks.remove(index);
        }
    }

    pub fn validate_task(&self, task_name: &str) -> bool {
        self.tasks.contains(&String::from(task_name))
    }

    pub fn list_tasks(&self) -> Vec<String> {
        self.tasks.clone()
    }

    pub fn clear_tasks(&mut self) {
        self.tasks.clear();
    }
}
