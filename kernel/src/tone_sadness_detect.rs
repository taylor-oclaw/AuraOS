extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ToneSadnessDetect {
    data: Vec<u8>,
}

impl ToneSadnessDetect {
    pub fn new() -> Self {
        ToneSadnessDetect { data: Vec::new() }
    }

    pub fn add_data(&mut self, byte: u8) {
        self.data.push(byte);
    }

    pub fn get_data_length(&self) -> usize {
        self.data.len()
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn is_sadness_detected(&self) -> bool {
        // Dummy logic for demonstration purposes
        // In a real scenario, this would involve complex AI analysis
        self.data.iter().any(|&b| b == 42)
    }

    pub fn get_data_as_string(&self) -> String {
        let mut result = String::new();
        for &byte in &self.data {
            result.push_str(&format!("{:02X} ", byte));
        }
        result
    }
}
