extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct AnalyticsTrendAnalysis {
    data: Vec<f64>,
}

impl AnalyticsTrendAnalysis {
    pub fn new() -> Self {
        AnalyticsTrendAnalysis { data: Vec::new() }
    }

    pub fn add_data(&mut self, value: f64) {
        self.data.push(value);
    }

    pub fn get_average(&self) -> Option<f64> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.data.iter().sum::<f64>() / self.data.len() as f64)
        }
    }

    pub fn get_max(&self) -> Option<f64> {
        self.data.iter().cloned().max()
    }

    pub fn get_min(&self) -> Option<f64> {
        self.data.iter().cloned().min()
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }
}
