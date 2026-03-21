extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ParentalSafeSearch {
    blocked_keywords: Vec<String>,
}

impl ParentalSafeSearch {
    pub fn new() -> Self {
        ParentalSafeSearch {
            blocked_keywords: Vec::new(),
        }
    }

    pub fn add_blocked_keyword(&mut self, keyword: &str) {
        self.blocked_keywords.push(String::from(keyword));
    }

    pub fn remove_blocked_keyword(&mut self, keyword: &str) -> bool {
        if let Some(index) = self.blocked_keywords.iter().position(|k| k == keyword) {
            self.blocked_keywords.remove(index);
            true
        } else {
            false
        }
    }

    pub fn is_keyword_blocked(&self, keyword: &str) -> bool {
        self.blocked_keywords.contains(&String::from(keyword))
    }

    pub fn list_blocked_keywords(&self) -> Vec<String> {
        self.blocked_keywords.clone()
    }

    pub fn clear_all_keywords(&mut self) {
        self.blocked_keywords.clear();
    }
}
