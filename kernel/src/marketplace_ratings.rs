extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MarketplaceRatings {
    ratings: Vec<(String, u8)>,
}

impl MarketplaceRatings {
    pub fn new() -> Self {
        MarketplaceRatings {
            ratings: Vec::new(),
        }
    }

    pub fn add_rating(&mut self, product_name: String, rating: u8) {
        if rating > 0 && rating <= 5 {
            self.ratings.push((product_name, rating));
        }
    }

    pub fn get_average_rating(&self, product_name: &str) -> Option<f32> {
        let mut total = 0;
        let mut count = 0;

        for (name, rating) in &self.ratings {
            if name == product_name {
                total += *rating as u32;
                count += 1;
            }
        }

        if count > 0 {
            Some(total as f32 / count as f32)
        } else {
            None
        }
    }

    pub fn get_all_ratings(&self) -> &Vec<(String, u8)> {
        &self.ratings
    }

    pub fn remove_product(&mut self, product_name: &str) {
        self.ratings.retain(|(name, _)| name != product_name);
    }

    pub fn top_rated_products(&self, limit: usize) -> Vec<(String, u8)> {
        let mut sorted_ratings = self.ratings.clone();
        sorted_ratings.sort_by_key(|&(_, rating)| -rating as i32);

        sorted_ratings.into_iter().take(limit).collect()
    }
}
