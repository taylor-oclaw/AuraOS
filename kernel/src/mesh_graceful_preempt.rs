extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn mesh_graceful_preempt_init() {
    // Initialization logic for the module
}

pub extern "C" fn mesh_graceful_preempt_exit() {
    // Cleanup logic for the module
}

pub struct MeshGracefulPreempt {
    tasks: Vec<String>,
    preempt_count: usize,
}

impl MeshGracefulPreempt {
    pub fn new() -> Self {
        MeshGracefulPreempt {
            tasks: Vec::new(),
            preempt_count: 0,
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

    pub fn list_tasks(&self) -> Vec<String> {
        self.tasks.clone()
    }

    pub fn preempt_task(&mut self, task_name: &str) -> bool {
        if let Some(index) = self.tasks.iter().position(|t| t == task_name) {
            self.preempt_count += 1;
            // Simulate preemption logic here
            true
        } else {
            false
        }
    }

    pub fn get_preempt_count(&self) -> usize {
        self.preempt_count
    }
}
