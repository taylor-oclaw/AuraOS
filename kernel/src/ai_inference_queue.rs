extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct AIInferenceQueue {
    queue: Vec<String>,
}

impl AIInferenceQueue {
    pub fn new() -> Self {
        AIInferenceQueue {
            queue: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, task: String) {
        self.queue.push(task);
    }

    pub fn dequeue(&mut self) -> Option<String> {
        if self.queue.is_empty() {
            None
        } else {
            Some(self.queue.remove(0))
        }
    }

    pub fn peek(&self) -> Option<&String> {
        self.queue.first()
    }

    pub fn size(&self) -> usize {
        self.queue.len()
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_operations() {
        let mut queue = AIInferenceQueue::new();

        assert_eq!(queue.size(), 0);
        assert!(queue.peek().is_none());

        queue.enqueue(String::from("Task1"));
        queue.enqueue(String::from("Task2"));

        assert_eq!(queue.size(), 2);
        assert_eq!(queue.peek(), Some(&String::from("Task1")));

        let task = queue.dequeue();
        assert_eq!(task, Some(String::from("Task1")));
        assert_eq!(queue.size(), 1);

        queue.clear();
        assert_eq!(queue.size(), 0);
        assert!(queue.peek().is_none());
    }
}
