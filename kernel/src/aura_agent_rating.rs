extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialization code if needed
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Cleanup code if needed
}

pub struct AuraAgentRating {
    agent_id: String,
    ratings: Vec<u8>,
}

impl AuraAgentRating {
    pub fn new(agent_id: &str) -> Self {
        AuraAgentRating {
            agent_id: String::from(agent_id),
            ratings: Vec::new(),
        }
    }

    pub fn add_rating(&mut self, rating: u8) {
        if rating > 0 && rating <= 100 {
            self.ratings.push(rating);
        }
    }

    pub fn get_average_rating(&self) -> Option<u8> {
        if self.ratings.is_empty() {
            None
        } else {
            let total: u32 = self.ratings.iter().map(|&r| r as u32).sum();
            Some((total / self.ratings.len() as u32) as u8)
        }
    }

    pub fn get_max_rating(&self) -> Option<u8> {
        self.ratings.iter().cloned().max()
    }

    pub fn get_min_rating(&self) -> Option<u8> {
        self.ratings.iter().cloned().min()
    }

    pub fn get_ratings_count(&self) -> usize {
        self.ratings.len()
    }
}
