extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct EntropyPool {
    pool: Vec<u8>,
    capacity: usize,
}

impl EntropyPool {
    pub fn new(capacity: usize) -> Self {
        EntropyPool {
            pool: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn add_entropy(&mut self, data: &[u8]) {
        if self.pool.len() + data.len() > self.capacity {
            let to_add = self.capacity - self.pool.len();
            self.pool.extend_from_slice(&data[..to_add]);
        } else {
            self.pool.extend_from_slice(data);
        }
    }

    pub fn get_entropy(&mut self, amount: usize) -> Vec<u8> {
        if amount > self.pool.len() {
            let result = self.pool.clone();
            self.pool.clear();
            result
        } else {
            let result = self.pool.split_off(self.pool.len() - amount);
            result
        }
    }

    pub fn clear(&mut self) {
        self.pool.clear();
    }

    pub fn is_full(&self) -> bool {
        self.pool.len() == self.capacity
    }

    pub fn entropy_size(&self) -> usize {
        self.pool.len()
    }
}

#[no_mangle]
pub extern "C" fn rust_stop() {
    // Cleanup code if necessary
}
