extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AuraAgentReview {
    reviews: Vec<String>,
}

impl AuraAgentReview {
    pub fn new() -> Self {
        AuraAgentReview {
            reviews: Vec::new(),
        }
    }

    pub fn add_review(&mut self, review: String) {
        self.reviews.push(review);
    }

    pub fn get_reviews(&self) -> &Vec<String> {
        &self.reviews
    }

    pub fn remove_review(&mut self, index: usize) -> Option<String> {
        if index < self.reviews.len() {
            Some(self.reviews.remove(index))
        } else {
            None
        }
    }

    pub fn get_average_length(&self) -> usize {
        if self.reviews.is_empty() {
            0
        } else {
            let total_length: usize = self.reviews.iter().map(|r| r.len()).sum();
            total_length / self.reviews.len()
        }
    }

    pub fn contains_keyword(&self, keyword: &str) -> bool {
        self.reviews.iter().any(|review| review.contains(keyword))
    }
}
