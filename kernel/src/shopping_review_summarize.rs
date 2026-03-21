extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ShoppingReviewSummarizer {
    reviews: Vec<String>,
}

impl ShoppingReviewSummarizer {
    pub fn new() -> Self {
        ShoppingReviewSummarizer {
            reviews: Vec::new(),
        }
    }

    pub fn add_review(&mut self, review: String) {
        self.reviews.push(review);
    }

    pub fn get_reviews_count(&self) -> usize {
        self.reviews.len()
    }

    pub fn get_all_reviews(&self) -> &Vec<String> {
        &self.reviews
    }

    pub fn summarize_reviews(&self) -> String {
        let mut summary = String::from("Summary of reviews:\n");
        for review in &self.reviews {
            summary.push_str(review);
            summary.push('\n');
        }
        summary
    }

    pub fn clear_reviews(&mut self) {
        self.reviews.clear();
    }
}
