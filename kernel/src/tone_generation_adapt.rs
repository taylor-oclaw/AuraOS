extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ToneIndustryJargonMatcher {
    jargon_list: Vec<String>,
}

impl ToneIndustryJargonMatcher {
    pub fn new() -> Self {
        ToneIndustryJargonMatcher {
            jargon_list: Vec::new(),
        }
    }

    pub fn add_jargon(&mut self, jargon: &str) {
        self.jargon_list.push(String::from(jargon));
    }

    pub fn remove_jargon(&mut self, jargon: &str) -> bool {
        if let Some(index) = self.jargon_list.iter().position(|j| *j == jargon) {
            self.jargon_list.remove(index);
            true
        } else {
            false
        }
    }

    pub fn contains_jargon(&self, text: &str) -> bool {
        for jargon in &self.jargon_list {
            if text.contains(jargon) {
                return true;
            }
        }
        false
    }

    pub fn list_jargons(&self) -> Vec<String> {
        self.jargon_list.clone()
    }

    pub fn clear_jargons(&mut self) {
        self.jargon_list.clear();
    }
}
