extern crate alloc;
use alloc::string::String;
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
    memory: Vec<u8>,
    free_lists: Vec<Vec<usize>>,
}

impl BuddyAllocator {
    pub fn new(size: usize) -> Self {
        let mut memory = vec![0; size];
        let levels = log2_ceil(size);
        let mut free_lists = vec![Vec::new(); levels];

        // Initialize the largest block
        free_lists[levels - 1].push(0);

        BuddyAllocator { memory, free_lists }
    }

    pub fn allocate(&mut self, size: usize) -> Option<usize> {
        let level = log2_ceil(size);
        if level >= self.free_lists.len() {
            return None;
        }

        // Find the first available block of at least the required size
        for i in level..self.free_lists.len() {
            if let Some(&block) = self.free_lists[i].first() {
                // Split blocks until we reach the desired level
                for j in (level..i).rev() {
                    let buddy = block + (1 << j);
                    self.free_lists[j].push(buddy);
                }
                self.free_lists[level].remove(0);
                return Some(block);
            }
        }

        None
    }

    pub fn deallocate(&mut self, address: usize, size: usize) {
        let level = log2_ceil(size);
        if level >= self.free_lists.len() {
            return;
        }

        // Coalesce buddies until we reach the largest possible block
        let mut current_address = address;
        for i in level..self.free_lists.len() {
            let buddy = current_address ^ (1 << i);
            if self.is_free(buddy, 1 << i) {
                self.remove_from_list(&mut self.free_lists[i], buddy);
                current_address = buddy.min(current_address);
            } else {
                break;
            }
        }

        // Insert the coalesced block back into the free list
        self.free_lists[level].push(current_address);
    }

    fn is_free(&self, address: usize, size: usize) -> bool {
        let level = log2_ceil(size);
        if level >= self.free_lists.len() {
            return false;
        }
        self.free_lists[level].contains(&(address))
    }

    fn remove_from_list(&mut self, list: &mut Vec<usize>, address: usize) {
        let index = list.iter().position(|&x| x == address).unwrap();
        list.remove(index);
    }

    pub fn total_memory(&self) -> usize {
        self.memory.len()
    }

    pub fn free_memory(&self) -> usize {
        self.free_lists.iter().map(|list| list.len() * (1 << self.free_lists.len())).sum()
    }
}
