extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct RelLossDetect {
    data: Vec<f32>,
    labels: Vec<u8>,
    predictions: Vec<f32>,
}

impl RelLossDetect {
    pub fn new(data: Vec<f32>, labels: Vec<u8>) -> Self {
        let predictions = vec![0.0; data.len() / 10]; // Assuming each label corresponds to 10 data points
        RelLossDetect { data, labels, predictions }
    }

    pub fn set_predictions(&mut self, predictions: Vec<f32>) {
        if predictions.len() == self.data.len() / 10 {
            self.predictions = predictions;
        } else {
            // Handle error or panic
        }
    }

    pub fn calculate_loss(&self) -> f32 {
        let mut total_loss = 0.0;
        for i in 0..self.labels.len() {
            let label_index = i * 10;
            let predicted_label = self.predictions[i];
            let true_label = self.labels[i] as f32;
            let loss = (predicted_label - true_label).abs();
            total_loss += loss;
        }
        total_loss / self.labels.len() as f32
    }

    pub fn update_predictions(&mut self, new_predictions: Vec<f32>) {
        if new_predictions.len() == self.predictions.len() {
            for i in 0..self.predictions.len() {
                self.predictions[i] = new_predictions[i];
            }
        } else {
            // Handle error or panic
        }
    }

    pub fn get_accuracy(&self) -> f32 {
        let mut correct = 0;
        for i in 0..self.labels.len() {
            let label_index = i * 10;
            let predicted_label = self.predictions[i];
            let true_label = self.labels[i] as f32;
            if (predicted_label - true_label).abs() < 0.5 {
                correct += 1;
            }
        }
        correct as f32 / self.labels.len() as f32
    }

    pub fn reset(&mut self) {
        self.predictions = vec![0.0; self.data.len() / 10];
    }
}
