extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct BatchScheduler {
    tasks: Vec<String>,
}

impl BatchScheduler {
    pub fn new() -> Self {
        BatchScheduler { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task: String) {
        self.tasks.push(task);
    }

    pub fn remove_task(&mut self, index: usize) -> Option<String> {
        if index < self.tasks.len() {
            Some(self.tasks.remove(index))
        } else {
            None
        }
    }

    pub fn get_task_count(&self) -> usize {
        self.tasks.len()
    }

    pub fn list_tasks(&self) -> Vec<&String> {
        self.tasks.iter().collect()
    }

    pub fn clear_tasks(&mut self) {
        self.tasks.clear();
    }
}
