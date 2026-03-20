extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct WebPCodec {
    data: Vec<u8>,
}

impl WebPCodec {
    pub fn new(data: Vec<u8>) -> Self {
        WebPCodec { data }
    }

    pub fn encode(&self, quality: u8) -> Result<Vec<u8>, String> {
        // Simulate encoding logic
        if self.data.is_empty() {
            return Err(String::from("No data to encode"));
        }
        if quality > 100 {
            return Err(String::from("Quality must be between 0 and 100"));
        }

        // Dummy encoding: just append the quality as a byte
        let mut encoded_data = self.data.clone();
        encoded_data.push(quality);
        Ok(encoded_data)
    }

    pub fn decode(&self) -> Result<Vec<u8>, String> {
        // Simulate decoding logic
        if self.data.is_empty() {
            return Err(String::from("No data to decode"));
        }
        let mut decoded_data = self.data.clone();
        decoded_data.pop(); // Remove the dummy quality byte
        Ok(decoded_data)
    }

    pub fn get_size(&self) -> usize {
        self.data.len()
    }

    pub fn is_valid(&self) -> bool {
        !self.data.is_empty() && self.data[0] == 0x52 && self.data[1] == 0x49 && self.data[2] == 0x46 && self.data[3] == 0x46
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }
}
