extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut wifi_pattern = PredictAppWifiPattern::new();
    wifi_pattern.train(&[1, 2, 3, 4, 5]);
    let prediction = wifi_pattern.predict_next();
}

pub struct PredictAppWifiPattern {
    data: Vec<u8>,
}

impl PredictAppWifiPattern {
    pub fn new() -> Self {
        PredictAppWifiPattern { data: Vec::new() }
    }

    pub fn train(&mut self, pattern: &[u8]) {
        self.data.extend_from_slice(pattern);
    }

    pub fn predict_next(&self) -> u8 {
        if self.data.is_empty() {
            return 0;
        }
        *self.data.last().unwrap()
    }

    pub fn get_pattern_length(&self) -> usize {
        self.data.len()
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn append_to_pattern(&mut self, value: u8) {
        self.data.push(value);
    }
}
