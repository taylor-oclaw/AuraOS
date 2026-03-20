extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MemoryPool {
    pool: Vec<u8>,
    free_blocks: Vec<usize>,
}

impl MemoryPool {
    pub fn new(size: usize) -> Self {
        let mut pool = vec![0; size];
        let free_blocks = vec![0];
        MemoryPool { pool, free_blocks }
    }

    pub fn allocate(&mut self, size: usize) -> Option<usize> {
        for &start in &self.free_blocks {
            if start + size <= self.pool.len() {
                self.free_blocks.retain(|&x| x != start);
                return Some(start);
            }
        }
        None
    }

    pub fn deallocate(&mut self, start: usize) {
        self.free_blocks.push(start);
    }

    pub fn get_free_space(&self) -> usize {
        let mut free_space = 0;
        for &start in &self.free_blocks {
            if start + 1 < self.pool.len() {
                free_space += self.pool.len() - start;
            }
        }
        free_space
    }

    pub fn is_allocated(&self, address: usize) -> bool {
        !self.free_blocks.contains(&address)
    }

    pub fn get_pool_size(&self) -> usize {
        self.pool.len()
    }
}
