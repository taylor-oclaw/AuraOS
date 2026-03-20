extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MarketplaceReviews {
    reviews: Vec<(String, String)>, // (product_name, review_text)
}

impl MarketplaceReviews {
    pub fn new() -> Self {
        MarketplaceReviews {
            reviews: Vec::new(),
        }
    }

    pub fn add_review(&mut self, product_name: &str, review_text: &str) {
        let product_name = String::from(product_name);
        let review_text = String::from(review_text);
        self.reviews.push((product_name, review_text));
    }

    pub fn get_reviews_for_product(&self, product_name: &str) -> Vec<&String> {
        self.reviews
            .iter()
            .filter(|&&(ref prod, _)| prod == product_name)
            .map(|(_, review)| review)
            .collect()
    }

    pub fn count_reviews(&self) -> usize {
        self.reviews.len()
    }

    pub fn get_all_products(&self) -> Vec<&String> {
        let mut products: Vec<String> = self
            .reviews
            .iter()
            .map(|(product, _)| product.clone())
            .collect();
        products.dedup();
        products.iter().collect()
    }

    pub fn remove_reviews_for_product(&mut self, product_name: &str) {
        self.reviews.retain(|(prod, _)| prod != product_name);
    }
}
