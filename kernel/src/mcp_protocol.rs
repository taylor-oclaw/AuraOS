extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MCPProtocol {
    data: Vec<u8>,
    header: String,
}

impl MCPProtocol {
    pub fn new(header: &str) -> Self {
        MCPProtocol {
            data: Vec::new(),
            header: String::from(header),
        }
    }

    pub fn add_data(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }

    pub fn get_header(&self) -> &str {
        &self.header
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn data_length(&self) -> usize {
        self.data.len()
    }
}
