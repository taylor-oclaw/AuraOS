extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PredictWebsitePrerender {
    predictions: Vec<String>,
}

impl PredictWebsitePrerender {
    pub fn new() -> Self {
        PredictWebsitePrerender {
            predictions: Vec::new(),
        }
    }

    pub fn add_prediction(&mut self, prediction: String) {
        self.predictions.push(prediction);
    }

    pub fn get_predictions(&self) -> &Vec<String> {
        &self.predictions
    }

    pub fn clear_predictions(&mut self) {
        self.predictions.clear();
    }

    pub fn has_prediction(&self, prediction: &str) -> bool {
        self.predictions.iter().any(|p| p == prediction)
    }

    pub fn remove_prediction(&mut self, prediction: &str) {
        if let Some(index) = self.predictions.iter().position(|p| p == prediction) {
            self.predictions.remove(index);
        }
    }
}
