extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AsfAdapterNative {
    data: Vec<u8>,
    name: String,
}

impl AsfAdapterNative {
    pub fn new(name: &str) -> Self {
        AsfAdapterNative {
            data: Vec::new(),
            name: String::from(name),
        }
    }

    pub fn add_data(&mut self, byte: u8) {
        self.data.push(byte);
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

    pub fn set_name(&mut self, new_name: &str) {
        self.name = String::from(new_name);
    }
}
