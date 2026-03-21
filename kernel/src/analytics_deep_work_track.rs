extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct AnalyticsDeepWorkTrack {
    tasks: Vec<String>,
    completed_tasks: usize,
}

impl AnalyticsDeepWorkTrack {
    pub fn new() -> Self {
        AnalyticsDeepWorkTrack {
            tasks: Vec::new(),
            completed_tasks: 0,
        }
    }

    pub fn add_task(&mut self, task: String) {
        self.tasks.push(task);
    }

    pub fn complete_task(&mut self) -> Option<String> {
        if let Some(task) = self.tasks.pop() {
            self.completed_tasks += 1;
            Some(task)
        } else {
            None
        }
    }

    pub fn get_total_tasks(&self) -> usize {
        self.tasks.len()
    }

    pub fn get_completed_tasks(&self) -> usize {
        self.completed_tasks
    }

    pub fn list_pending_tasks(&self) -> Vec<String> {
        self.tasks.clone()
    }
}
