extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut allocator = HugePageAllocator::new();
    allocator.initialize(1024);
    allocator.allocate_page();
    allocator.allocate_pages(3);
    allocator.free_page(0);
    allocator.free_pages(vec![1, 2]);
}

pub struct HugePageAllocator {
    pages: Vec<u64>,
    free_list: Vec<usize>,
}

impl HugePageAllocator {
    pub fn new() -> Self {
        HugePageAllocator {
            pages: Vec::new(),
            free_list: Vec::new(),
        }
    }

    pub fn initialize(&mut self, num_pages: usize) {
        for i in 0..num_pages {
            self.pages.push(0);
            self.free_list.push(i);
        }
    }

    pub fn allocate_page(&mut self) -> Option<usize> {
        if let Some(index) = self.free_list.pop() {
            self.pages[index] = 1;
            Some(index)
        } else {
            None
        }
    }

    pub fn allocate_pages(&mut self, num_pages: usize) -> Vec<Option<usize>> {
        let mut allocated_pages = Vec::new();
        for _ in 0..num_pages {
            allocated_pages.push(self.allocate_page());
        }
        allocated_pages
    }

    pub fn free_page(&mut self, index: usize) {
        if index < self.pages.len() && self.pages[index] == 1 {
            self.pages[index] = 0;
            self.free_list.push(index);
        }
    }

    pub fn free_pages(&mut self, indices: Vec<usize>) {
        for &index in &indices {
            self.free_page(index);
        }
    }
}
