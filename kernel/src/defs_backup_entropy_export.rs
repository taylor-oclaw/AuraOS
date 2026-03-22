extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct EntropyBackup {
    data: Vec<u8>,
    capacity: usize,
}

impl EntropyBackup {
    pub const fn new(capacity: usize) -> Self {
        EntropyBackup {
            data: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn add_entropy(&mut self, entropy: &[u8]) -> Result<(), &'static str> {
        if self.data.len() + entropy.len() > self.capacity {
            Err("Capacity exceeded")
        } else {
            self.data.extend_from_slice(entropy);
            Ok(())
        }
    }

    pub fn get_entropy(&self) -> &[u8] {
        &self.data
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn is_full(&self) -> bool {
        self.data.len() == self.capacity
    }

    pub fn remaining_capacity(&self) -> usize {
        self.capacity - self.data.len()
    }
}