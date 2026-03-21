extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ContextConfidenceScore {
    context: String,
    scores: Vec<f32>,
}

impl ContextConfidenceScore {
    pub fn new(context: &str) -> Self {
        ContextConfidenceScore {
            context: String::from(context),
            scores: Vec::new(),
        }
    }

    pub fn add_score(&mut self, score: f32) {
        self.scores.push(score);
    }

    pub fn average_score(&self) -> Option<f32> {
        if self.scores.is_empty() {
            None
        } else {
            let sum: f32 = self.scores.iter().sum();
            Some(sum / self.scores.len() as f32)
        }
    }

    pub fn highest_score(&self) -> Option<f32> {
        self.scores.iter().cloned().max()
    }

    pub fn lowest_score(&self) -> Option<f32> {
        self.scores.iter().cloned().min()
    }

    pub fn clear_scores(&mut self) {
        self.scores.clear();
    }
}
