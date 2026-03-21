extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn runtime_swift_init() {
    // Initialization logic for the module
}

pub extern "C" fn runtime_swift_exit() {
    // Cleanup logic for the module
}

pub struct RuntimeSwift {
    tasks: Vec<String>,
    current_task_index: usize,
}

impl RuntimeSwift {
    pub fn new() -> Self {
        RuntimeSwift {
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
        self.tasks.get(self.current_task_index)
    }

    pub fn switch_to_next_task(&mut self) {
        if !self.tasks.is_empty() {
            self.current_task_index = (self.current_task_index + 1) % self.tasks.len();
        }
    }

    pub fn list_tasks(&self) -> Vec<&String> {
        self.tasks.iter().collect()
    }
}
