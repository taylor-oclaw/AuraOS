extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PriorityQueue<T> {
    elements: Vec<(u32, T)>,
}

impl<T> PriorityQueue<T> {
    pub fn new() -> Self {
        PriorityQueue { elements: Vec::new() }
    }

    pub fn push(&mut self, priority: u32, item: T) {
        self.elements.push((priority, item));
        self.elements.sort_by(|a, b| b.0.cmp(&a.0)); // Sort by descending order of priority
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop().map(|(_, item)| item)
    }

    pub fn peek(&self) -> Option<&T> {
        self.elements.last().map(|&(_, ref item)| item)
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_queue() {
        let mut pq = PriorityQueue::new();
        assert!(pq.is_empty());
        assert_eq!(pq.len(), 0);

        pq.push(1, "low");
        pq.push(3, "high");
        pq.push(2, "medium");

        assert!(!pq.is_empty());
        assert_eq!(pq.len(), 3);
        assert_eq!(pq.peek(), Some(&"high"));

        assert_eq!(pq.pop(), Some("high"));
        assert_eq!(pq.pop(), Some("medium"));
        assert_eq!(pq.pop(), Some("low"));
        assert_eq!(pq.pop(), None);

        assert!(pq.is_empty());
        assert_eq!(pq.len(), 0);
    }
}
