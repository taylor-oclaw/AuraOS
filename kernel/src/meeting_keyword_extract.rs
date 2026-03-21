extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeetingKeywordExtractor {
    keywords: Vec<String>,
}

impl MeetingKeywordExtractor {
    pub fn new() -> Self {
        MeetingKeywordExtractor {
            keywords: Vec::new(),
        }
    }

    pub fn add_keyword(&mut self, keyword: String) {
        if !self.keywords.contains(&keyword) {
            self.keywords.push(keyword);
        }
    }

    pub fn remove_keyword(&mut self, keyword: &str) -> bool {
        let pos = self.keywords.iter().position(|k| k == keyword);
        match pos {
            Some(index) => {
                self.keywords.remove(index);
                true
            }
            None => false,
        }
    }

    pub fn contains_keyword(&self, keyword: &str) -> bool {
        self.keywords.contains(&String::from(keyword))
    }

    pub fn list_keywords(&self) -> Vec<String> {
        self.keywords.clone()
    }

    pub fn extract_keywords_from_text(&self, text: &str) -> Vec<String> {
        let mut found_keywords = Vec::new();
        for keyword in &self.keywords {
            if text.contains(keyword) {
                found_keywords.push(keyword.clone());
            }
        }
        found_keywords
    }
}
