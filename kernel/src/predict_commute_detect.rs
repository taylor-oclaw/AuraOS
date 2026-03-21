extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CommutePredictor {
    data: Vec<u8>,
    threshold: u32,
}

impl CommutePredictor {
    pub fn new(threshold: u32) -> Self {
        CommutePredictor {
            data: Vec::new(),
            threshold,
        }
    }

    pub fn add_data(&mut self, value: u8) {
        self.data.push(value);
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn set_threshold(&mut self, threshold: u32) {
        self.threshold = threshold;
    }

    pub fn predict_commute_time(&self) -> Option<u32> {
        if self.data.is_empty() {
            return None;
        }
        let sum: u32 = self.data.iter().map(|&x| x as u32).sum();
        Some(sum / self.data.len() as u32)
    }

    pub fn is_above_threshold(&self) -> bool {
        if let Some(avg_time) = self.predict_commute_time() {
            avg_time > self.threshold
        } else {
            false
        }
    }
}
