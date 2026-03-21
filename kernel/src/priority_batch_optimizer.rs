extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PriorityBatchOptimizer {
    tasks: Vec<Task>,
}

impl PriorityBatchOptimizer {
    pub fn new() -> Self {
        PriorityBatchOptimizer { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, name: String, priority: u32) {
        let task = Task { name, priority };
        self.tasks.push(task);
        self.tasks.sort_by_key(|t| t.priority);
    }

    pub fn remove_task(&mut self, name: &str) -> bool {
        if let Some(index) = self.tasks.iter().position(|t| t.name == name) {
            self.tasks.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_top_task(&self) -> Option<&Task> {
        self.tasks.first()
    }

    pub fn list_tasks(&self) -> Vec<String> {
        self.tasks.iter().map(|t| t.name.clone()).collect()
    }

    pub fn clear_tasks(&mut self) {
        self.tasks.clear();
    }
}

struct Task {
    name: String,
    priority: u32,
}
