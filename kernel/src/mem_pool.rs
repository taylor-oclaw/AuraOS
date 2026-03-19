extern crate alloc;
use alloc::vec::Vec;

const SLAB_SIZES: [usize; 6] = [32, 64, 128, 256, 512, 1024];

pub struct Slab {
    size: usize,
    blocks: Vec<*mut u8>,
    free: Vec<*mut u8>,
}

pub struct MemPool {
    slabs: Vec<Slab>,
    total_allocated: usize,
    total_freed: usize,
}

impl MemPool {
    pub fn new() -> Self {
        Self {
            slabs: Vec::new(),
            total_allocated: 0,
            total_freed: 0,
        }
    }

    pub fn alloc(&mut self, size: usize) -> Option<*mut u8> {
        let slab_size = SLAB_SIZES.iter().find(|&&s| s >= size)?;
        if let Some(slab_idx) = self.slabs.iter().position(|s| s.size == *slab_size) {
            if let Some(ptr) = self.slabs[slab_idx].free.pop() {
                self.total_allocated += 1;
                return Some(ptr);
            }
        }

        // Create a new slab if none is available
        let mut blocks = Vec::new();
        for _ in 0..8 { // Allocate 8 blocks per slab for simplicity
            let block = unsafe { alloc::alloc::alloc(alloc::alloc::Layout::from_size_align_unchecked(*slab_size, *slab_size)) };
            if block.is_null() {
                return None;
            }
            blocks.push(block);
        }

        let mut free = Vec::new();
        for block in &blocks {
            free.push(*block);
        }

        self.slabs.push(Slab { size: *slab_size, blocks, free });
        self.total_allocated += 1;
        Some(self.slabs.last_mut().unwrap().free.pop().unwrap())
    }

    pub fn free(&mut self, ptr: *mut u8, size: usize) {
        let slab_size = SLAB_SIZES.iter().find(|&&s| s >= size);
        if let Some(&sz) = slab_size {
            if let Some(slab) = self.slabs.iter_mut().find(|s| s.size == sz) {
                slab.free.push(ptr);
                self.total_freed += 1;
            }
        }
    }

    pub fn stats(&self) -> (usize, usize) {
        (self.total_allocated, self.total_freed)
    }
}
