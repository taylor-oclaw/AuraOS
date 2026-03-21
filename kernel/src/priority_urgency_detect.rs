extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn priority_urgency_detect_init() -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn priority_urgency_detect_exit() -> i32 {
    0
}

pub struct PriorityUrgencyDetect {
    tasks: Vec<Task>,
}

impl PriorityUrgencyDetect {
    pub fn new() -> Self {
        PriorityUrgencyDetect { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, name: String, priority: u32, urgency: u32) {
        let task = Task { name, priority, urgency };
        self.tasks.push(task);
    }

    pub fn remove_task(&mut self, name: &str) -> bool {
        if let Some(index) = self.tasks.iter().position(|t| t.name == name) {
            self.tasks.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_highest_priority_task(&self) -> Option<&Task> {
        self.tasks.iter().max_by_key(|t| t.priority)
    }

    pub fn get_most_urgent_task(&self) -> Option<&Task> {
        self.tasks.iter().max_by_key(|t| t.urgency)
    }

    pub fn list_tasks(&self) -> Vec<String> {
        self.tasks.iter().map(|t| t.name.clone()).collect()
    }
}

struct Task {
    name: String,
    priority: u32,
    urgency: u32,
}
