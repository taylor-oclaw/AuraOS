extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialization code for the module
}

pub extern "C" fn rust_ffi_exit() {
    // Cleanup code for the module
}

pub struct AuraAssistantAutoOrganize {
    tasks: Vec<String>,
}

impl AuraAssistantAutoOrganize {
    pub fn new() -> Self {
        AuraAssistantAutoOrganize { tasks: Vec::new() }
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

    pub fn get_tasks(&self) -> &[String] {
        &self.tasks
    }

    pub fn clear_tasks(&mut self) {
        self.tasks.clear();
    }

    pub fn find_task(&self, task: &str) -> Option<usize> {
        self.tasks.iter().position(|t| t == task)
    }
}
