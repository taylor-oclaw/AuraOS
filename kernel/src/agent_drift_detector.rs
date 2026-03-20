extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentDriftDetector {
    data: Vec<u8>,
    threshold: u32,
}

impl AgentDriftDetector {
    pub fn new(threshold: u32) -> Self {
        AgentDriftDetector {
            data: Vec::new(),
            threshold,
        }
    }

    pub fn add_data(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn calculate_variance(&self) -> u32 {
        if self.data.is_empty() {
            return 0;
        }
        let mean = self.calculate_mean();
        let mut variance = 0;
        for &value in &self.data {
            let diff = value as i32 - mean as i32;
            variance += (diff * diff) as u32;
        }
        variance / self.data.len() as u32
    }

    pub fn detect_drift(&self) -> bool {
        let variance = self.calculate_variance();
        variance > self.threshold
    }

    fn calculate_mean(&self) -> u32 {
        if self.data.is_empty() {
            return 0;
        }
        let sum: u32 = self.data.iter().map(|&x| x as u32).sum();
        sum / self.data.len() as u32
    }
}
