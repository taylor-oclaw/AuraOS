extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SpeechStutterWordPredict {
    // Example data structure for the module
    predictions: Vec<String>,
}

impl SpeechStutterWordPredict {
    pub fn new() -> Self {
        SpeechStutterWordPredict {
            predictions: Vec::new(),
        }
    }

    pub fn add_prediction(&mut self, word: &str) {
        self.predictions.push(word.to_string());
    }

    pub fn get_predictions(&self) -> &[String] {
        &self.predictions
    }

    pub fn clear_predictions(&mut self) {
        self.predictions.clear();
    }

    pub fn has_prediction(&self, word: &str) -> bool {
        self.predictions.contains(&word.to_string())
    }

    pub fn remove_prediction(&mut self, word: &str) {
        if let Some(index) = self.predictions.iter().position(|w| w == word) {
            self.predictions.remove(index);
        }
    }
}
