extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ActionItemAssignee {
    name: String,
    tasks: Vec<String>,
}

impl ActionItemAssignee {
    pub fn new(name: &str) -> Self {
        ActionItemAssignee {
            name: String::from(name),
            tasks: Vec::new(),
        }
    }

    pub fn add_task(&mut self, task: &str) {
        self.tasks.push(String::from(task));
    }

    pub fn remove_task(&mut self, task_index: usize) -> Option<String> {
        if task_index < self.tasks.len() {
            Some(self.tasks.remove(task_index))
        } else {
            None
        }
    }

    pub fn get_tasks(&self) -> &Vec<String> {
        &self.tasks
    }

    pub fn has_task(&self, task: &str) -> bool {
        self.tasks.contains(&String::from(task))
    }

    pub fn clear_tasks(&mut self) {
        self.tasks.clear();
    }
}
