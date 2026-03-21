extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PriorityPreemptHandler {
    tasks: Vec<Task>,
}

impl PriorityPreemptHandler {
    pub fn new() -> Self {
        PriorityPreemptHandler { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, name: String, priority: u32) {
        let task = Task { name, priority };
        self.tasks.push(task);
        self.tasks.sort_by_key(|t| t.priority);
    }

    pub fn remove_task(&mut self, name: &str) -> Option<Task> {
        if let Some(index) = self.tasks.iter().position(|t| t.name == name) {
            return Some(self.tasks.remove(index));
        }
        None
    }

    pub fn get_highest_priority_task(&self) -> Option<&Task> {
        self.tasks.first()
    }

    pub fn preempt_current_task(&mut self, current_name: &str) -> Option<Task> {
        if let Some(index) = self.tasks.iter().position(|t| t.name == current_name) {
            return Some(self.tasks.remove(index));
        }
        None
    }

    pub fn list_tasks(&self) -> Vec<&Task> {
        self.tasks.iter().collect()
    }
}

struct Task {
    name: String,
    priority: u32,
}
