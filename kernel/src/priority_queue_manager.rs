extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PriorityQueueManager {
    queue: Vec<(u32, String)>,
}

impl PriorityQueueManager {
    pub fn new() -> Self {
        PriorityQueueManager { queue: Vec::new() }
    }

    pub fn push(&mut self, priority: u32, item: String) {
        let entry = (priority, item);
        if let Some((index, _)) = self.queue.iter().enumerate().find(|&(_, &(p, _))| p > priority) {
            self.queue.insert(index, entry);
        } else {
            self.queue.push(entry);
        }
    }

    pub fn pop(&mut self) -> Option<String> {
        self.queue.pop().map(|(_, item)| item)
    }

    pub fn peek(&self) -> Option<&String> {
        self.queue.last().map(|&(_, ref item)| item)
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }
}
