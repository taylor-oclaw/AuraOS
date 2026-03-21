extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut recommendation = RecommendationDraft::new();
    recommendation.add_user("user1");
    recommendation.add_item("item1");
    recommendation.add_rating("user1", "item1", 5);
    recommendation.add_rating("user1", "item2", 3);
    recommendation.add_rating("user2", "item1", 4);

    let recommendations = recommendation.get_recommendations("user1");
    for item in recommendations {
    }

    loop {}
}

pub struct RecommendationDraft {
    users: Vec<String>,
    items: Vec<String>,
    ratings: Vec<(String, String, u8)>, // (user, item, rating)
}

impl RecommendationDraft {
    pub fn new() -> Self {
        RecommendationDraft {
            users: Vec::new(),
            items: Vec::new(),
            ratings: Vec::new(),
        }
    }

    pub fn add_user(&mut self, user: &str) {
        if !self.users.contains(&user.to_string()) {
            self.users.push(user.to_string());
        }
    }

    pub fn add_item(&mut self, item: &str) {
        if !self.items.contains(&item.to_string()) {
            self.items.push(item.to_string());
        }
    }

    pub fn add_rating(&mut self, user: &str, item: &str, rating: u8) {
        if self.users.contains(&user.to_string()) && self.items.contains(&item.to_string()) {
            self.ratings.push((user.to_string(), item.to_string(), rating));
        }
    }

    pub fn get_recommendations(&self, user: &str) -> Vec<String> {
        let mut recommendations = Vec::new();
        if let Some(user_index) = self.users.iter().position(|u| u == user) {
            for (i, &(ref other_user, ref item, _)) in self.ratings.iter().enumerate() {
                if i != user_index && !recommendations.contains(item) {
                    recommendations.push(item.clone());
                }
            }
        }
        recommendations
    }

    pub fn get_average_rating(&self, item: &str) -> Option<f32> {
        let mut total = 0;
        let mut count = 0;
        for &(ref _, ref i, rating) in self.ratings.iter() {
            if i == item {
                total += rating as u32;
                count += 1;
            }
        }
        if count > 0 {
            Some(total as f32 / count as f32)
        } else {
            None
        }
    }
}
