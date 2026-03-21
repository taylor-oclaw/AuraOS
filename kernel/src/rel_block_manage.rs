extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rel_block_manage_init() {
    // Initialization logic for the block management module
}

pub extern "C" fn rel_block_manage_exit() {
    // Cleanup logic for the block management module
}

pub struct BlockManager {
    blocks: Vec<u8>,
    free_blocks: Vec<usize>,
}

impl BlockManager {
    pub fn new(size: usize) -> Self {
        let mut blocks = vec![0; size];
        let free_blocks: Vec<usize> = (0..size).collect();
        BlockManager { blocks, free_blocks }
    }

    pub fn allocate_block(&mut self) -> Option<usize> {
        if let Some(index) = self.free_blocks.pop() {
            self.blocks[index] = 1; // Mark block as allocated
            Some(index)
        } else {
            None
        }
    }

    pub fn deallocate_block(&mut self, index: usize) {
        if index < self.blocks.len() && self.blocks[index] == 1 {
            self.blocks[index] = 0; // Mark block as free
            self.free_blocks.push(index);
        }
    }

    pub fn is_block_allocated(&self, index: usize) -> bool {
        index < self.blocks.len() && self.blocks[index] == 1
    }

    pub fn get_free_block_count(&self) -> usize {
        self.free_blocks.len()
    }

    pub fn get_total_block_size(&self) -> usize {
        self.blocks.len()
    }
}

pub extern "C" fn rel_block_manage_new(size: usize) -> *mut BlockManager {
    let manager = Box::new(BlockManager::new(size));
    Box::into_raw(manager)
}

pub extern "C" fn rel_block_manage_free(manager_ptr: *mut BlockManager) {
    if !manager_ptr.is_null() {
        unsafe { drop(Box::from_raw(manager_ptr)) };
    }
}

pub extern "C" fn rel_block_manage_allocate_block(manager_ptr: *mut BlockManager) -> usize {
    if manager_ptr.is_null() {
        return usize::MAX;
    }
    unsafe {
        (*manager_ptr).allocate_block().unwrap_or(usize::MAX)
    }
}

pub extern "C" fn rel_block_manage_deallocate_block(manager_ptr: *mut BlockManager, index: usize) {
    if !manager_ptr.is_null() {
        unsafe { (*manager_ptr).deallocate_block(index) };
    }
}

pub extern "C" fn rel_block_manage_is_block_allocated(manager_ptr: *const BlockManager, index: usize) -> bool {
    if manager_ptr.is_null() {
        return false;
    }
    unsafe { (*manager_ptr).is_block_allocated(index) }
}

pub extern "C" fn rel_block_manage_get_free_block_count(manager_ptr: *const BlockManager) -> usize {
    if manager_ptr.is_null() {
        return 0;
    }
    unsafe { (*manager_ptr).get_free_block_count() }
}

pub extern "C" fn rel_block_manage_get_total_block_size(manager_ptr: *const BlockManager) -> usize {
    if manager_ptr.is_null() {
        return 0;
    }
    unsafe { (*manager_ptr).get_total_block_size() }
}
