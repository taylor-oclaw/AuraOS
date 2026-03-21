extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ProfileModeSchedule {
    tasks: Vec<String>,
    current_task_index: usize,
}

impl ProfileModeSchedule {
    pub fn new() -> Self {
        ProfileModeSchedule {
            tasks: Vec::new(),
            current_task_index: 0,
        }
    }

    pub fn add_task(&mut self, task_name: &str) {
        self.tasks.push(String::from(task_name));
    }

    pub fn remove_task(&mut self, task_name: &str) -> bool {
        if let Some(index) = self.tasks.iter().position(|t| t == task_name) {
            self.tasks.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_current_task(&self) -> Option<&String> {
        if self.tasks.is_empty() {
            None
        } else {
            Some(&self.tasks[self.current_task_index])
        }
    }

    pub fn next_task(&mut self) {
        if !self.tasks.is_empty() {
            self.current_task_index = (self.current_task_index + 1) % self.tasks.len();
        }
    }

    pub fn previous_task(&mut self) {
        if !self.tasks.is_empty() {
            self.current_task_index = if self.current_task_index == 0 {
                self.tasks.len() - 1
            } else {
                self.current_task_index - 1
            };
        }
    }
}
