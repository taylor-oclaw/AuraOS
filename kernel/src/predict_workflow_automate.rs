extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct PredictWorkflowAutomate {
    tasks: Vec<String>,
    current_task_index: usize,
}

impl PredictWorkflowAutomate {
    pub fn new(tasks: Vec<String>) -> Self {
        PredictWorkflowAutomate {
            tasks,
            current_task_index: 0,
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

    pub fn get_current_task(&self) -> Option<&String> {
        if self.current_task_index < self.tasks.len() {
            Some(&self.tasks[self.current_task_index])
        } else {
            None
        }
    }

    pub fn next_task(&mut self) -> Option<&String> {
        if self.current_task_index + 1 < self.tasks.len() {
            self.current_task_index += 1;
            Some(&self.tasks[self.current_task_index])
        } else {
            None
        }
    }

    pub fn previous_task(&mut self) -> Option<&String> {
        if self.current_task_index > 0 {
            self.current_task_index -= 1;
            Some(&self.tasks[self.current_task_index])
        } else {
            None
        }
    }
}
