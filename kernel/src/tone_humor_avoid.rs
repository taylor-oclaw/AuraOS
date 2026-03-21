extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ToneHumorAvoid {
    // Example fields for the struct
    keywords: Vec<String>,
    tone_scores: Vec<i32>,
}

impl ToneHumorAvoid {
    pub fn new() -> Self {
        ToneHumorAvoid {
            keywords: Vec::new(),
            tone_scores: Vec::new(),
        }
    }

    pub fn add_keyword(&mut self, keyword: String) {
        self.keywords.push(keyword);
    }

    pub fn remove_keyword(&mut self, keyword: &str) -> bool {
        if let Some(index) = self.keywords.iter().position(|k| k == keyword) {
            self.keywords.remove(index);
            self.tone_scores.remove(index);
            true
        } else {
            false
        }
    }

    pub fn analyze_text(&self, text: &str) -> i32 {
        let mut score = 0;
        for keyword in &self.keywords {
            if text.contains(keyword) {
                score += self.tone_scores[self.keywords.iter().position(|k| k == keyword).unwrap()];
            }
        }
        score
    }

    pub fn set_tone_score(&mut self, keyword: &str, score: i32) -> bool {
        if let Some(index) = self.keywords.iter().position(|k| k == keyword) {
            self.tone_scores[index] = score;
            true
        } else {
            false
        }
    }

    pub fn get_keywords(&self) -> Vec<String> {
        self.keywords.clone()
    }
}
