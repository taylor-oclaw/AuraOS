extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct PriorityImpactScore {
    scores: Vec<u32>,
}

impl PriorityImpactScore {
    pub fn new(size: usize) -> Self {
        PriorityImpactScore {
            scores: vec![0; size],
        }
    }

    pub fn set_score(&mut self, index: usize, score: u32) -> Result<(), &'static str> {
        if index < self.scores.len() {
            self.scores[index] = score;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    pub fn get_score(&self, index: usize) -> Result<u32, &'static str> {
        if index < self.scores.len() {
            Ok(self.scores[index])
        } else {
            Err("Index out of bounds")
        }
    }

    pub fn total_score(&self) -> u32 {
        self.scores.iter().sum()
    }

    pub fn average_score(&self) -> Option<f32> {
        if self.scores.is_empty() {
            None
        } else {
            Some(self.total_score() as f32 / self.scores.len() as f32)
        }
    }

    pub fn highest_score(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }
}
