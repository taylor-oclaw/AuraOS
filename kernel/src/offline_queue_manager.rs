extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct OfflineQueueManager {
    queue: Vec<String>,
}

impl OfflineQueueManager {
    pub fn new() -> Self {
        OfflineQueueManager {
            queue: Vec::new(),
        }
    }

    pub fn add_task(&mut self, task: String) {
        self.queue.push(task);
    }

    pub fn remove_task(&mut self, index: usize) -> Option<String> {
        if index < self.queue.len() {
            Some(self.queue.remove(index))
        } else {
            None
        }
    }

    pub fn get_task(&self, index: usize) -> Option<&String> {
        self.queue.get(index)
    }

    pub fn task_count(&self) -> usize {
        self.queue.len()
    }

    pub fn clear_queue(&mut self) {
        self.queue.clear();
    }
}
