extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rel_memory_share_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn rel_memory_share_exit() {
    // Cleanup logic for the module
}

pub struct MemoryShare {
    data: Vec<u8>,
    size: usize,
}

impl MemoryShare {
    pub fn new(size: usize) -> Self {
        MemoryShare {
            data: vec![0; size],
            size,
        }
    }

    pub fn write(&mut self, offset: usize, buffer: &[u8]) -> Result<(), String> {
        if offset + buffer.len() > self.size {
            return Err(String::from("Write out of bounds"));
        }
        self.data[offset..offset + buffer.len()].copy_from_slice(buffer);
        Ok(())
    }

    pub fn read(&self, offset: usize, size: usize) -> Result<Vec<u8>, String> {
        if offset + size > self.size {
            return Err(String::from("Read out of bounds"));
        }
        Ok(self.data[offset..offset + size].to_vec())
    }

    pub fn resize(&mut self, new_size: usize) -> Result<(), String> {
        if new_size == 0 {
            return Err(String::from("Size cannot be zero"));
        }
        self.data.resize(new_size, 0);
        self.size = new_size;
        Ok(())
    }

    pub fn get_size(&self) -> usize {
        self.size
    }
}
