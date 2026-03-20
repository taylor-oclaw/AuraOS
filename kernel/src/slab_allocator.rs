extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct SlabAllocator {
    slabs: Vec<Slab>,
}

impl SlabAllocator {
    pub fn new() -> Self {
        SlabAllocator {
            slabs: Vec::new(),
        }
    }

    pub fn allocate(&mut self, size: usize) -> Option<usize> {
        for slab in &mut self.slabs {
            if let Some(offset) = slab.allocate(size) {
                return Some(slab.base + offset);
            }
        }
        None
    }

    pub fn deallocate(&mut self, address: usize) {
        for slab in &mut self.slabs {
            if slab.contains(address) {
                slab.deallocate(address - slab.base);
                break;
            }
        }
    }

    pub fn add_slab(&mut self, base: usize, size: usize) {
        self.slabs.push(Slab::new(base, size));
    }

    pub fn total_allocated(&self) -> usize {
        self.slabs.iter().map(|s| s.allocated).sum()
    }

    pub fn total_free(&self) -> usize {
        self.slabs.iter().map(|s| s.free()).sum()
    }
}

struct Slab {
    base: usize,
    size: usize,
    bitmap: Vec<u8>,
    allocated: usize,
}

impl Slab {
    fn new(base: usize, size: usize) -> Self {
        let bitmap_size = (size / 64 + if size % 64 != 0 { 1 } else { 0 }) as usize;
        Slab {
            base,
            size,
            bitmap: vec![0; bitmap_size],
            allocated: 0,
        }
    }

    fn allocate(&mut self, size: usize) -> Option<usize> {
        if size > self.size || self.free() < size {
            return None;
        }

        let mut offset = 0;
        while offset + size <= self.size {
            let start_bit = offset / 64;
            let end_bit = (offset + size - 1) / 64;

            if self.is_free_range(start_bit, end_bit) {
                for i in start_bit..=end_bit {
                    self.bitmap[i] |= (1 << ((i == start_bit) as usize * 63)) | ((1 << (((i == end_bit) as usize * 63) + size - 1)) - 1);
                }
                self.allocated += size;
                return Some(offset);
            }

            offset += 64;
        }

        None
    }

    fn deallocate(&mut self, offset: usize) {
        let start_bit = offset / 64;
        let end_bit = (offset + 63) / 64;

        for i in start_bit..=end_bit {
            self.bitmap[i] &= !(1 << ((i == start_bit) as usize * 63)) & !((1 << (((i == end_bit) as usize * 63) + 63)) - 1);
        }
        self.allocated -= 64;
    }

    fn contains(&self, address: usize) -> bool {
        address >= self.base && address < self.base + self.size
    }

    fn free(&self) -> usize {
        self.size - self.allocated
    }

    fn is_free_range(&self, start_bit: usize, end_bit: usize) -> bool {
        for i in start_bit..=end_bit {
            if self.bitmap[i] != 0 {
                return false;
            }
        }
        true
    }
}
