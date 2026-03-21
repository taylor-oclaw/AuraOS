extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn priority_effort_estimate_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn priority_effort_estimate_exit() {
    // Cleanup logic for the module
}

pub struct PriorityEffortEstimate {
    tasks: Vec<Task>,
}

impl PriorityEffortEstimate {
    pub fn new() -> Self {
        PriorityEffortEstimate { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, name: String, priority: u32, effort: u32) {
        let task = Task { name, priority, effort };
        self.tasks.push(task);
    }

    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn remove_task(&mut self, index: usize) -> Option<Task> {
        if index < self.tasks.len() {
            Some(self.tasks.remove(index))
        } else {
            None
        }
    }

    pub fn total_effort(&self) -> u32 {
        self.tasks.iter().map(|task| task.effort).sum()
    }
}

struct Task {
    name: String,
    priority: u32,
    effort: u32,
}
