extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct AIInferenceQueue {
    queue: Vec<String>,
}

impl AIInferenceQueue {
    pub fn new() -> Self {
        AIInferenceQueue { queue: Vec::new() }
    }

    pub fn enqueue(&mut self, task: String) {
        self.queue.push(task);
    }

    pub fn dequeue(&mut self) -> Option<String> {
        self.queue.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn get_task(&self, index: usize) -> Option<&String> {
        self.queue.get(index)
    }
}