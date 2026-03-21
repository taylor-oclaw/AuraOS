extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct AiSecConfidenceScore {
    scores: Vec<f32>,
    labels: Vec<String>,
}

impl AiSecConfidenceScore {
    pub fn new(scores: Vec<f32>, labels: Vec<String>) -> Self {
        AiSecConfidenceScore { scores, labels }
    }

    pub fn add_score(&mut self, score: f32, label: String) {
        self.scores.push(score);
        self.labels.push(label);
    }

    pub fn get_highest_confidence(&self) -> Option<(f32, &str)> {
        if self.scores.is_empty() || self.labels.is_empty() {
            return None;
        }
        let mut max_score = 0.0;
        let mut max_label = "";
        for (score, label) in self.scores.iter().zip(self.labels.iter()) {
            if *score > max_score {
                max_score = *score;
                max_label = label;
            }
        }
        Some((max_score, max_label))
    }

    pub fn get_average_confidence(&self) -> Option<f32> {
        if self.scores.is_empty() {
            return None;
        }
        let total: f32 = self.scores.iter().sum();
        Some(total / self.scores.len() as f32)
    }

    pub fn remove_low_confidence_scores(&mut self, threshold: f32) {
        self.scores.retain(|&score| score >= threshold);
        self.labels.retain(|_, _| true); // Retain all labels to maintain alignment
    }

    pub fn get_label_count(&self, label: &str) -> usize {
        self.labels.iter().filter(|&&l| l == label).count()
    }
}
