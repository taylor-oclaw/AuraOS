extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshStorageShare {
    data: Vec<u8>,
}

impl MeshStorageShare {
    pub fn new() -> Self {
        MeshStorageShare { data: Vec::new() }
    }

    pub fn store_data(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }

    pub fn retrieve_data(&self) -> &[u8] {
        &self.data
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn append_string(&mut self, s: &str) {
        self.data.extend_from_slice(s.as_bytes());
    }

    pub fn get_string(&self) -> Option<&str> {
        core::str::from_utf8(&self.data).ok()
    }
}
