extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn tone_excitement_match_init() {
    // Initialization logic for the module
}

pub extern "C" fn tone_excitement_match_exit() {
    // Cleanup logic for the module
}

pub struct ToneExcitementMatch {
    keywords: Vec<String>,
    scores: Vec<u32>,
}

impl ToneExcitementMatch {
    pub fn new(keywords: Vec<&str>) -> Self {
        let mut scores = Vec::new();
        for _ in 0..keywords.len() {
            scores.push(0);
        }
        ToneExcitementMatch {
            keywords: keywords.into_iter().map(|s| s.to_string()).collect(),
            scores,
        }
    }

    pub fn add_keyword(&mut self, keyword: &str) {
        self.keywords.push(keyword.to_string());
        self.scores.push(0);
    }

    pub fn remove_keyword(&mut self, index: usize) -> Option<String> {
        if index < self.keywords.len() {
            self.scores.remove(index);
            Some(self.keywords.remove(index))
        } else {
            None
        }
    }

    pub fn update_score(&mut self, index: usize, score: u32) -> bool {
        if index < self.scores.len() {
            self.scores[index] = score;
            true
        } else {
            false
        }
    }

    pub fn get_highest_scoring_keyword(&self) -> Option<&String> {
        self.keywords.iter().max_by_key(|&k| self.get_score(k))
    }

    fn get_score(&self, keyword: &str) -> u32 {
        if let Some(index) = self.keywords.iter().position(|k| k == keyword) {
            self.scores[index]
        } else {
            0
        }
    }
}
