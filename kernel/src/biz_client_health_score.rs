extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct BizClientHealthScore {
    client_id: String,
    scores: Vec<u8>,
}

impl BizClientHealthScore {
    pub fn new(client_id: &str) -> Self {
        BizClientHealthScore {
            client_id: String::from(client_id),
            scores: Vec::new(),
        }
    }

    pub fn add_score(&mut self, score: u8) {
        if score <= 100 {
            self.scores.push(score);
        }
    }

    pub fn average_score(&self) -> Option<u8> {
        if self.scores.is_empty() {
            None
        } else {
            let total: u32 = self.scores.iter().map(|&s| s as u32).sum();
            Some((total / self.scores.len() as u32) as u8)
        }
    }

    pub fn highest_score(&self) -> Option<u8> {
        self.scores.iter().cloned().max()
    }

    pub fn lowest_score(&self) -> Option<u8> {
        self.scores.iter().cloned().min()
    }

    pub fn clear_scores(&mut self) {
        self.scores.clear();
    }
}
