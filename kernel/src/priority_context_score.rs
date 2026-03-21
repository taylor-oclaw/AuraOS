extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct PriorityContextScore {
    scores: Vec<u32>,
    total_score: u32,
}

impl PriorityContextScore {
    pub fn new() -> Self {
        PriorityContextScore {
            scores: Vec::new(),
            total_score: 0,
        }
    }

    pub fn add_score(&mut self, score: u32) {
        self.scores.push(score);
        self.total_score += score;
    }

    pub fn remove_score(&mut self, index: usize) -> Option<u32> {
        if let Some(score) = self.scores.get(index).cloned() {
            self.scores.remove(index);
            self.total_score -= score;
            Some(score)
        } else {
            None
        }
    }

    pub fn get_total_score(&self) -> u32 {
        self.total_score
    }

    pub fn average_score(&self) -> Option<f32> {
        if self.scores.is_empty() {
            None
        } else {
            Some(self.total_score as f32 / self.scores.len() as f32)
        }
    }

    pub fn highest_score(&self) -> Option<u32> {
        self.scores.iter().cloned().max()
    }
}
