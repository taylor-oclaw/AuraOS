extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct PriorityImportanceScore {
    tasks: Vec<Task>,
}

impl PriorityImportanceScore {
    pub fn new() -> Self {
        PriorityImportanceScore { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, name: String, priority: u32, importance: u32) {
        let task = Task { name, priority, importance };
        self.tasks.push(task);
    }

    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn remove_task_by_name(&mut self, name: &str) {
        self.tasks.retain(|task| task.name != name);
    }

    pub fn update_priority(&mut self, name: &str, new_priority: u32) {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.name == name) {
            task.priority = new_priority;
        }
    }

    pub fn update_importance(&mut self, name: &str, new_importance: u32) {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.name == name) {
            task.importance = new_importance;
        }
    }
}

struct Task {
    name: String,
    priority: u32,
    importance: u32,
}
