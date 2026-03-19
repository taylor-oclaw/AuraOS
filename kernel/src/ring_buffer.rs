extern crate alloc;
use alloc::vec::Vec;

pub struct RingBuffer<T> {
    buf: Vec<Option<T>>,
    head: usize,
    tail: usize,
    count: usize,
    capacity: usize,
}

impl<T: Clone> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut buf = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buf.push(None);
        }
        Self {
            buf,
            head: 0,
            tail: 0,
            count: 0,
            capacity,
        }
    }

    pub fn push(&mut self, item: T) -> bool {
        if self.count == self.capacity {
            return false;
        }
        self.buf[self.tail] = Some(item);
        self.tail = (self.tail + 1) % self.capacity;
        self.count += 1;
        true
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        let item = self.buf[self.head].take();
        self.head = (self.head + 1) % self.capacity;
        self.count -= 1;
        item
    }

    pub fn peek(&self) -> Option<&T> {
        self.buf[self.head].as_ref()
    }

    pub fn is_full(&self) -> bool {
        self.count == self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}
