extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ParentalAgeRating {
    age_ratings: Vec<(String, u8)>,
}

impl ParentalAgeRating {
    pub fn new() -> Self {
        ParentalAgeRating {
            age_ratings: Vec::new(),
        }
    }

    pub fn add_rating(&mut self, title: String, rating: u8) {
        self.age_ratings.push((title, rating));
    }

    pub fn get_rating(&self, title: &str) -> Option<u8> {
        for (t, r) in &self.age_ratings {
            if t == title {
                return Some(*r);
            }
        }
        None
    }

    pub fn remove_rating(&mut self, title: &str) {
        self.age_ratings.retain(|(t, _)| t != title);
    }

    pub fn list_titles(&self) -> Vec<String> {
        self.age_ratings.iter().map(|(t, _)| t.clone()).collect()
    }

    pub fn count_ratings(&self) -> usize {
        self.age_ratings.len()
    }
}
