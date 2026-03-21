extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ToneFrustrationDetect {
    keywords: Vec<String>,
    threshold: usize,
}

impl ToneFrustrationDetect {
    pub fn new(keywords: Vec<&str>, threshold: usize) -> Self {
        let keywords = keywords.into_iter().map(|s| s.to_string()).collect();
        ToneFrustrationDetect { keywords, threshold }
    }

    pub fn add_keyword(&mut self, keyword: &str) {
        self.keywords.push(keyword.to_string());
    }

    pub fn remove_keyword(&mut self, keyword: &str) -> bool {
        if let Some(pos) = self.keywords.iter().position(|k| k == keyword) {
            self.keywords.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn set_threshold(&mut self, threshold: usize) {
        self.threshold = threshold;
    }

    pub fn detect_frustration(&self, text: &str) -> bool {
        let mut count = 0;
        for keyword in &self.keywords {
            if text.contains(keyword) {
                count += 1;
                if count >= self.threshold {
                    return true;
                }
            }
        }
        false
    }

    pub fn list_keywords(&self) -> Vec<String> {
        self.keywords.clone()
    }
}
