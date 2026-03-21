extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ProfileDataPartition {
    data: Vec<u8>,
}

impl ProfileDataPartition {
    pub fn new() -> Self {
        ProfileDataPartition { data: Vec::new() }
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

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for byte in &self.data {
            result.push_str(&String::from("info"));
        }
        result
    }
}
