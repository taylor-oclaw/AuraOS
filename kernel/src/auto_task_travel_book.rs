extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AutoTaskTravelBook {
    tasks: Vec<String>,
}

impl AutoTaskTravelBook {
    pub fn new() -> Self {
        AutoTaskTravelBook { tasks: Vec::new() }
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

    pub fn get_task(&self, index: usize) -> Option<&String> {
        self.tasks.get(index)
    }

    pub fn list_tasks(&self) -> &Vec<String> {
        &self.tasks
    }

    pub fn clear_tasks(&mut self) {
        self.tasks.clear();
    }
}
