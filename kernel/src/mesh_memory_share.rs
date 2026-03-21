extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshMemoryShare {
    data: Vec<u8>,
}

impl MeshMemoryShare {
    pub fn new() -> Self {
        MeshMemoryShare { data: Vec::new() }
    }

    pub fn add_data(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn data_size(&self) -> usize {
        self.data.len()
    }

    pub fn append_string(&mut self, s: &str) {
        self.data.extend_from_slice(s.as_bytes());
    }
}
