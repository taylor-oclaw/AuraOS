extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct DeadlockDetector {
    lock_count: AtomicUsize,
    lock_order: Vec<usize>,
}

impl DeadlockDetector {
    pub fn new() -> Self {
        DeadlockDetector {
            lock_count: AtomicUsize::new(0),
            lock_order: Vec::new(),
        }
    }

    pub fn acquire_lock(&self, lock_id: usize) -> bool {
        let current_count = self.lock_count.load(Ordering::SeqCst);
        if current_count > 0 && !self.is_valid_order(lock_id) {
            return false;
        }
        self.lock_order.push(lock_id);
        self.lock_count.fetch_add(1, Ordering::SeqCst);
        true
    }

    pub fn release_lock(&self, lock_id: usize) -> bool {
        if let Some(index) = self.lock_order.iter().position(|&id| id == lock_id) {
            self.lock_order.remove(index);
            self.lock_count.fetch_sub(1, Ordering::SeqCst);
            true
        } else {
            false
        }
    }

    pub fn is_valid_order(&self, lock_id: usize) -> bool {
        for &id in &self.lock_order {
            if id >= lock_id {
                return false;
            }
        }
        true
    }

    pub fn get_lock_count(&self) -> usize {
        self.lock_count.load(Ordering::SeqCst)
    }

    pub fn get_lock_order(&self) -> Vec<usize> {
        self.lock_order.clone()
    }
}
