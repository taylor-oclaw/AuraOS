extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AnalyticsProductivityScore {
    scores: Vec<u32>,
    total_score: u32,
    count: usize,
}

impl AnalyticsProductivityScore {
    pub fn new() -> Self {
        AnalyticsProductivityScore {
            scores: Vec::new(),
            total_score: 0,
            count: 0,
        }
    }

    pub fn add_score(&mut self, score: u32) {
        self.scores.push(score);
        self.total_score += score;
        self.count += 1;
    }

    pub fn average_score(&self) -> Option<u32> {
        if self.count == 0 {
            None
        } else {
            Some(self.total_score / self.count as u32)
        }
    }

    pub fn highest_score(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn lowest_score(&self) -> Option<u32> {
        self.scores.iter().min().copied()
    }

    pub fn total_scores(&self) -> u32 {
        self.total_score
    }
}
