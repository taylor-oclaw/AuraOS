extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct OfflineRetryEngine {
    tasks: Vec<String>,
    max_retries: usize,
}

impl OfflineRetryEngine {
    pub fn new(max_retries: usize) -> Self {
        OfflineRetryEngine {
            tasks: Vec::new(),
            max_retries,
        }
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

    pub fn retry_task(&mut self, index: usize) -> bool {
        if index < self.tasks.len() && self.max_retries > 0 {
            // Simulate task retry logic
            true
        } else {
            false
        }
    }

    pub fn list_tasks(&self) -> Vec<String> {
        self.tasks.clone()
    }
}
