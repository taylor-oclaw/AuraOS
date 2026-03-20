extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct TieredStorage {
    fast_storage: Vec<u8>,
    slow_storage: Vec<u8>,
}

impl TieredStorage {
    pub fn new(fast_size: usize, slow_size: usize) -> Self {
        TieredStorage {
            fast_storage: vec![0; fast_size],
            slow_storage: vec![0; slow_size],
        }
    }

    pub fn read(&self, offset: usize, size: usize) -> Option<Vec<u8>> {
        if offset + size > self.fast_storage.len() {
            return None;
        }
        Some(self.fast_storage[offset..offset + size].to_vec())
    }

    pub fn write(&mut self, offset: usize, data: &[u8]) -> bool {
        if offset + data.len() > self.fast_storage.len() {
            return false;
        }
        self.fast_storage[offset..offset + data.len()].copy_from_slice(data);
        true
    }

    pub fn move_to_slow(&mut self, offset: usize, size: usize) -> bool {
        if offset + size > self.fast_storage.len() || size > self.slow_storage.len() {
            return false;
        }
        let data = self.read(offset, size).unwrap();
        self.slow_storage[..size].copy_from_slice(&data);
        true
    }

    pub fn move_to_fast(&mut self, offset: usize, size: usize) -> bool {
        if offset + size > self.slow_storage.len() || size > self.fast_storage.len() {
            return false;
        }
        let data = &self.slow_storage[offset..offset + size];
        self.write(offset, data);
        true
    }

    pub fn clear(&mut self) {
        self.fast_storage.fill(0);
        self.slow_storage.fill(0);
    }
}
