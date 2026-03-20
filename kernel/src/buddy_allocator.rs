extern crate alloc;
use alloc::vec::Vec;
use alloc::vec;

fn log2_ceil(n: usize) -> usize {
    if n <= 1 { return 0; }
    let mut v = n - 1;
    let mut r = 0;
    while v > 0 { v >>= 1; r += 1; }
    r
}

pub struct BuddyAllocator {
    levels: usize,
    free_lists: Vec<Vec<usize>>,
    total_size: usize,
}

impl BuddyAllocator {
    pub fn new(size: usize) -> Self {
        let levels = log2_ceil(size) + 1;
        let mut free_lists = Vec::with_capacity(levels);
        for _ in 0..levels {
            free_lists.push(Vec::new());
        }
        if levels > 0 {
            free_lists[levels - 1].push(0);
        }
        BuddyAllocator { levels, free_lists, total_size: size }
    }

    pub fn allocate(&mut self, size: usize) -> Option<usize> {
        let level = log2_ceil(size);
        if level >= self.levels { return None; }
        for l in level..self.levels {
            if !self.free_lists[l].is_empty() {
                let block = self.free_lists[l].remove(0);
                let mut current_level = l;
                while current_level > level {
                    current_level -= 1;
                    let buddy = block + (1 << current_level);
                    self.free_lists[current_level].push(buddy);
                }
                return Some(block);
            }
        }
        None
    }

    pub fn deallocate(&mut self, addr: usize, size: usize) {
        let level = log2_ceil(size);
        if level < self.levels {
            self.free_lists[level].push(addr);
        }
    }

    pub fn available_blocks(&self) -> usize {
        let mut count = 0;
        for list in &self.free_lists {
            count += list.len();
        }
        count
    }

    pub fn total_size(&self) -> usize {
        self.total_size
    }
}
