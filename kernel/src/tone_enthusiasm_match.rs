extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct ToneEnthusiasmMatch {
    keywords: Vec<String>,
    scores: Vec<i32>,
}

impl ToneEnthusiasmMatch {
    pub fn new(keywords: Vec<&str>) -> Self {
        let mut scores = Vec::new();
        for _ in keywords.iter() {
            scores.push(0);
        }
        ToneEnthusiasmMatch {
            keywords: keywords.into_iter().map(|s| s.to_string()).collect(),
            scores,
        }
    }

    pub fn add_keyword(&mut self, keyword: &str) {
        self.keywords.push(keyword.to_string());
        self.scores.push(0);
    }

    pub fn remove_keyword(&mut self, keyword: &str) {
        if let Some(index) = self.keywords.iter().position(|k| k == keyword) {
            self.keywords.remove(index);
            self.scores.remove(index);
        }
    }

    pub fn update_score(&mut self, keyword: &str, score: i32) {
        if let Some(index) = self.keywords.iter().position(|k| k == keyword) {
            self.scores[index] = score;
        }
    }

    pub fn get_score(&self, keyword: &str) -> Option<i32> {
        self.keywords.iter().position(|k| k == keyword).map(|index| self.scores[index])
    }

    pub fn top_keyword(&self) -> Option<&String> {
        self.scores
            .iter()
            .enumerate()
            .max_by_key(|&(_, &score)| score)
            .map(|(index, _)| &self.keywords[index])
    }
}
