extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AiPipelineExecutor {
    tasks: Vec<String>,
    current_task_index: usize,
}

impl AiPipelineExecutor {
    pub fn new() -> Self {
        AiPipelineExecutor {
            tasks: Vec::new(),
            current_task_index: 0,
        }
    }

    pub fn add_task(&mut self, task: String) {
        self.tasks.push(task);
    }

    pub fn get_current_task(&self) -> Option<&String> {
        if self.current_task_index < self.tasks.len() {
            Some(&self.tasks[self.current_task_index])
        } else {
            None
        }
    }

    pub fn execute_next_task(&mut self) -> Option<String> {
        if self.current_task_index < self.tasks.len() {
            let task = self.tasks[self.current_task_index].clone();
            self.current_task_index += 1;
            Some(task)
        } else {
            None
        }
    }

    pub fn reset_pipeline(&mut self) {
        self.current_task_index = 0;
    }

    pub fn get_task_count(&self) -> usize {
        self.tasks.len()
    }
}
