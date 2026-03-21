extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PredictWebsitePreload {
    predictions: Vec<String>,
}

impl PredictWebsitePreload {
    pub fn new() -> Self {
        PredictWebsitePreload {
            predictions: Vec::new(),
        }
    }

    pub fn add_prediction(&mut self, prediction: String) {
        self.predictions.push(prediction);
    }

    pub fn remove_prediction(&mut self, index: usize) -> Option<String> {
        if index < self.predictions.len() {
            Some(self.predictions.remove(index))
        } else {
            None
        }
    }

    pub fn get_predictions(&self) -> &Vec<String> {
        &self.predictions
    }

    pub fn clear_predictions(&mut self) {
        self.predictions.clear();
    }

    pub fn has_prediction(&self, prediction: &str) -> bool {
        self.predictions.contains(&String::from(prediction))
    }
}
