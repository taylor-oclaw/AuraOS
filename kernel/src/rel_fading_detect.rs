extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct RelFadingDetect {
    data: Vec<u8>,
    threshold: u8,
}

impl RelFadingDetect {
    pub fn new(data: Vec<u8>, threshold: u8) -> Self {
        RelFadingDetect { data, threshold }
    }

    pub fn update_data(&mut self, new_data: Vec<u8>) {
        self.data = new_data;
    }

    pub fn set_threshold(&mut self, new_threshold: u8) {
        self.threshold = new_threshold;
    }

    pub fn detect_fading(&self) -> bool {
        for &value in &self.data {
            if value < self.threshold {
                return true;
            }
        }
        false
    }

    pub fn average_value(&self) -> Option<u8> {
        if self.data.is_empty() {
            return None;
        }
        let sum: u32 = self.data.iter().map(|&v| v as u32).sum();
        Some((sum / self.data.len() as u32) as u8)
    }

    pub fn max_value(&self) -> Option<u8> {
        self.data.iter().cloned().max()
    }
}
