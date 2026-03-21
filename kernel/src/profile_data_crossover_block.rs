extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ProfileDataCrossoverBlock {
    data: Vec<u8>,
    name: String,
}

impl ProfileDataCrossoverBlock {
    pub fn new(name: &str) -> Self {
        ProfileDataCrossoverBlock {
            data: Vec::new(),
            name: String::from(name),
        }
    }

    pub fn add_data(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_data_size(&self) -> usize {
        self.data.len()
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut serialized = Vec::new();
        serialized.extend_from_slice(self.name.as_bytes());
        serialized.push(0); // Null terminator for the string
        serialized.extend_from_slice(&self.data);
        serialized
    }
}
