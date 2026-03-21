extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AutoTaskCalendar {
    tasks: Vec<Task>,
}

impl AutoTaskCalendar {
    pub fn new() -> Self {
        AutoTaskCalendar { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn remove_task(&mut self, index: usize) -> Option<Task> {
        if index < self.tasks.len() {
            Some(self.tasks.remove(index))
        } else {
            None
        }
    }

    pub fn get_task(&self, index: usize) -> Option<&Task> {
        self.tasks.get(index)
    }

    pub fn list_tasks(&self) -> &[Task] {
        &self.tasks
    }

    pub fn clear_tasks(&mut self) {
        self.tasks.clear();
    }
}

pub struct Task {
    name: String,
    description: String,
    due_date: u64, // Unix timestamp in seconds
}

impl Task {
    pub fn new(name: String, description: String, due_date: u64) -> Self {
        Task { name, description, due_date }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_due_date(&self) -> u64 {
        self.due_date
    }
}
