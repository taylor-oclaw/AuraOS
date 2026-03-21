extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct PredictAppTimePattern {
    patterns: Vec<String>,
    predictions: Vec<f64>,
}

impl PredictAppTimePattern {
    pub fn new() -> Self {
        PredictAppTimePattern {
            patterns: Vec::new(),
            predictions: Vec::new(),
        }
    }

    pub fn add_pattern(&mut self, pattern: String) {
        self.patterns.push(pattern);
    }

    pub fn get_patterns(&self) -> &Vec<String> {
        &self.patterns
    }

    pub fn predict_time(&mut self, pattern_index: usize) -> Result<f64, &'static str> {
        if pattern_index < self.predictions.len() {
            Ok(self.predictions[pattern_index])
        } else {
            Err("Pattern index out of bounds")
        }
    }

    pub fn update_prediction(&mut self, pattern_index: usize, prediction: f64) -> Result<(), &'static str> {
        if pattern_index < self.predictions.len() {
            self.predictions[pattern_index] = prediction;
            Ok(())
        } else {
            Err("Pattern index out of bounds")
        }
    }

    pub fn add_prediction(&mut self, prediction: f64) {
        self.predictions.push(prediction);
    }
}
