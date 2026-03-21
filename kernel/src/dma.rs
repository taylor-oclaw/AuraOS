extern crate alloc;
use alloc::vec::Vec;

pub struct DmaBuffer {
    pub phys_addr: u64,
    pub virt_addr: *mut u8,
    pub size: usize,
}

pub struct DmaAllocator {
    buffers: Vec<DmaBuffer>,
    next_phys: u64,
}

impl DmaAllocator {
    pub fn new(base_phys: u64) -> Self {
        DmaAllocator {
            buffers: Vec::new(),
            next_phys: base_phys,
        }
    }

    pub fn allocate(&mut self, size: usize) -> Option<&DmaBuffer> {
        let aligned_size = (size + 0xFFF) & !0xFFF; // Align to 4096
        let phys_addr = self.next_phys;
        let virt_addr = unsafe { alloc::alloc::alloc(alloc::alloc::Layout::from_size_align(aligned_size, 4096).unwrap()) as *mut u8 };

        if virt_addr.is_null() {
            return None;
        }

        let buffer = DmaBuffer {
            phys_addr,
            virt_addr,
            size: aligned_size,
        };

        self.buffers.push(buffer);
        self.next_phys += aligned_size as u64;

        Some(self.buffers.last().unwrap())
    }

    pub fn free(&mut self, phys_addr: u64) {
        if let Some(index) = self.buffers.iter().position(|b| b.phys_addr == phys_addr) {
            let buffer = self.buffers.remove(index);
            unsafe { alloc::alloc::dealloc(buffer.virt_addr, alloc::alloc::Layout::from_size_align(buffer.size, 4096).unwrap()) };
        }
    }

    pub fn total_allocated(&self) -> usize {
        self.buffers.iter().map(|b| b.size).sum()
    }
}
