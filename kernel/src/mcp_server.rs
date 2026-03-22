extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct McpServer {
    data: Vec<u8>,
}

impl McpServer {
    pub fn new() -> Self {
        McpServer { data: Vec::new() }
    }

    pub fn add_data(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn update_data(&mut self, new_data: &[u8]) {
        self.data.clear();
        self.add_data(new_data);
    }

    pub fn delete_data(&mut self, index: usize) {
        if index < self.data.len() {
            self.data.remove(index);
        }
    }

    pub fn get_size(&self) -> usize {
        self.data.len()
    }
}