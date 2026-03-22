extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ParallelDecode {
    data: Vec<u8>,
    decoded_data: Vec<String>,
}

impl ParallelDecode {
    pub fn new(data: Vec<u8>) -> Self {
        ParallelDecode {
            data,
            decoded_data: Vec::new(),
        }
    }

    pub fn decode(&mut self) {
        for byte in &self.data {
            let character = *byte as char;
            self.decoded_data.push(character.to_string());
        }
    }

    pub fn get_decoded_data(&self) -> &[String] {
        &self.decoded_data
    }

    pub fn clear_decoded_data(&mut self) {
        self.decoded_data.clear();
    }

    pub fn append_data(&mut self, additional_data: Vec<u8>) {
        self.data.extend(additional_data);
    }

    pub fn remove_data(&mut self, index: usize) -> Option<u8> {
        if index < self.data.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }
}