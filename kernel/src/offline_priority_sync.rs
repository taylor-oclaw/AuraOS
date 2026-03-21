extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct OfflinePrioritySync {
    tasks: Vec<Task>,
}

impl OfflinePrioritySync {
    pub fn new() -> Self {
        OfflinePrioritySync { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, name: String, priority: u32) {
        let task = Task { name, priority };
        self.tasks.push(task);
        self.tasks.sort_by_key(|t| t.priority);
    }

    pub fn remove_task(&mut self, name: &str) -> bool {
        if let Some(index) = self.tasks.iter().position(|t| t.name == name) {
            self.tasks.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_task_by_name(&self, name: &str) -> Option<&Task> {
        self.tasks.iter().find(|t| t.name == name)
    }

    pub fn list_tasks(&self) -> Vec<String> {
        self.tasks.iter().map(|t| t.name.clone()).collect()
    }

    pub fn get_highest_priority_task(&self) -> Option<&Task> {
        self.tasks.first()
    }
}

struct Task {
    name: String,
    priority: u32,
}
