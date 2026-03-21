extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshBatchScheduleWindow {
    tasks: Vec<String>,
    current_index: usize,
}

impl MeshBatchScheduleWindow {
    pub fn new() -> Self {
        MeshBatchScheduleWindow {
            tasks: Vec::new(),
            current_index: 0,
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
        if self.current_index < self.tasks.len() {
            Some(&self.tasks[self.current_index])
        } else {
            None
        }
    }

    pub fn next_task(&mut self) -> Option<&String> {
        if self.current_index < self.tasks.len() {
            let current = &self.tasks[self.current_index];
            self.current_index += 1;
            Some(current)
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.current_index = 0;
    }
}
