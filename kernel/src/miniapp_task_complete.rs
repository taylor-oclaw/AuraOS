extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MiniAppTaskComplete {
    tasks: Vec<String>,
    completed_tasks: Vec<String>,
}

impl MiniAppTaskComplete {
    pub fn new() -> Self {
        MiniAppTaskComplete {
            tasks: Vec::new(),
            completed_tasks: Vec::new(),
        }
    }

    pub fn add_task(&mut self, task: String) {
        self.tasks.push(task);
    }

    pub fn complete_task(&mut self, task_index: usize) -> Option<String> {
        if let Some(task) = self.tasks.get(task_index).cloned() {
            self.tasks.remove(task_index);
            self.completed_tasks.push(task.clone());
            Some(task)
        } else {
            None
        }
    }

    pub fn get_all_tasks(&self) -> &Vec<String> {
        &self.tasks
    }

    pub fn get_completed_tasks(&self) -> &Vec<String> {
        &self.completed_tasks
    }

    pub fn clear_completed_tasks(&mut self) {
        self.completed_tasks.clear();
    }
}
