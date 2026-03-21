extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct MiniAppScoreUpdate {
    scores: Vec<(String, u32)>,
}

impl MiniAppScoreUpdate {
    pub fn new() -> Self {
        MiniAppScoreUpdate {
            scores: Vec::new(),
        }
    }

    pub fn add_score(&mut self, name: String, score: u32) {
        self.scores.push((name, score));
    }

    pub fn get_score(&self, name: &str) -> Option<u32> {
        self.scores.iter().find(|&&(ref n, _)| n == name).map(|&(_, s)| s)
    }

    pub fn update_score(&mut self, name: String, new_score: u32) -> bool {
        for (n, s) in &mut self.scores {
            if *n == name {
                *s = new_score;
                return true;
            }
        }
        false
    }

    pub fn remove_score(&mut self, name: &str) -> Option<u32> {
        let pos = self.scores.iter().position(|&(ref n, _)| n == name);
        pos.map(|p| self.scores.remove(p).1)
    }

    pub fn top_scores(&self, count: usize) -> Vec<(String, u32)> {
        let mut sorted_scores = self.scores.clone();
        sorted_scores.sort_by_key(|&(_, score)| core::cmp::Reverse(score));
        sorted_scores.into_iter().take(count).collect()
    }
}
