extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct SharedMemory {
    data: Vec<u8>,
}

impl SharedMemory {
    pub fn new(size: usize) -> Self {
        SharedMemory {
            data: vec![0; size],
        }
    }

    pub fn read(&self, offset: usize, length: usize) -> Option<Vec<u8>> {
        if offset + length > self.data.len() {
            None
        } else {
            Some(self.data[offset..offset + length].to_vec())
        }
    }

    pub fn write(&mut self, offset: usize, data: &[u8]) -> bool {
        if offset + data.len() > self.data.len() {
            false
        } else {
            self.data[offset..offset + data.len()].copy_from_slice(data);
            true
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        for byte in self.data.iter_mut() {
            *byte = 0;
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
}
