extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AppAutomationWait {
    tasks: Vec<String>,
    completed_tasks: usize,
}

impl AppAutomationWait {
    pub fn new() -> Self {
        AppAutomationWait {
            tasks: Vec::new(),
            completed_tasks: 0,
        }
    }

    pub fn add_task(&mut self, task: String) {
        self.tasks.push(task);
    }

    pub fn get_total_tasks(&self) -> usize {
        self.tasks.len()
    }

    pub fn mark_task_completed(&mut self) {
        if self.completed_tasks < self.tasks.len() {
            self.completed_tasks += 1;
        }
    }

    pub fn get_completed_tasks(&self) -> usize {
        self.completed_tasks
    }

    pub fn has_pending_tasks(&self) -> bool {
        self.completed_tasks < self.tasks.len()
    }
}
