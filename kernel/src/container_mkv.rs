extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ContainerMKV {
    data: Vec<u8>,
}

impl ContainerMKV {
    pub fn new() -> Self {
        ContainerMKV { data: Vec::new() }
    }

    pub fn add_data(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn remove_data(&mut self, start: usize, len: usize) {
        if start + len <= self.data.len() {
            self.data = self.data.split_at_mut(start).1.split_at(len).0;
        }
    }

    pub fn get_size(&self) -> usize {
        self.data.len()
    }
}