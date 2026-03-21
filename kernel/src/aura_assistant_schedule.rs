extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialize the module if needed
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Clean up the module if needed
}

pub struct Schedule {
    tasks: Vec<Task>,
}

impl Schedule {
    pub fn new() -> Self {
        Schedule { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, name: String, priority: u8) {
        let task = Task { name, priority };
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

    pub fn list_tasks(&self) -> Vec<String> {
        self.tasks.iter().map(|task| task.name.clone()).collect()
    }

    pub fn sort_by_priority(&mut self) {
        self.tasks.sort_by_key(|task| task.priority);
    }
}

struct Task {
    name: String,
    priority: u8,
}
