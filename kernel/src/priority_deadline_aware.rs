extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PriorityDeadlineAware {
    tasks: Vec<Task>,
}

impl PriorityDeadlineAware {
    pub fn new() -> Self {
        PriorityDeadlineAware {
            tasks: Vec::new(),
        }
    }

    pub fn add_task(&mut self, name: String, priority: u32, deadline: u64) {
        let task = Task { name, priority, deadline };
        self.tasks.push(task);
    }

    pub fn remove_task(&mut self, index: usize) -> Option<Task> {
        if index < self.tasks.len() {
            Some(self.tasks.remove(index))
        } else {
            None
        }
    }

    pub fn get_task_by_name(&self, name: &str) -> Option<&Task> {
        self.tasks.iter().find(|task| task.name == name)
    }

    pub fn get_highest_priority_task(&self) -> Option<&Task> {
        self.tasks.iter().max_by_key(|task| task.priority)
    }

    pub fn is_within_deadline(&self, current_time: u64) -> bool {
        self.tasks.iter().all(|task| task.deadline >= current_time)
    }
}

struct Task {
    name: String,
    priority: u32,
    deadline: u64,
}
