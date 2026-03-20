extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PostProcessor {
    data: Vec<u8>,
}

impl PostProcessor {
    pub fn new() -> Self {
        PostProcessor { data: Vec::new() }
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

    pub fn data_length(&self) -> usize {
        self.data.len()
    }

    pub fn process_data(&mut self) -> Result<(), &'static str> {
        if self.data.is_empty() {
            return Err("No data to process");
        }
        // Example processing: reverse the data
        self.data.reverse();
        Ok(())
    }
}
