extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct SkillReviewSystem {
    skills: Vec<String>,
    users: Vec<String>,
    reviews: Vec<(String, String, String)>, // (user, skill, review)
}

impl SkillReviewSystem {
    pub fn new() -> Self {
        SkillReviewSystem {
            skills: Vec::new(),
            users: Vec::new(),
            reviews: Vec::new(),
        }
    }

    pub fn add_skill(&mut self, skill_name: &str) {
        if !self.skills.contains(&skill_name.to_string()) {
            self.skills.push(skill_name.to_string());
        }
    }

    pub fn remove_skill(&mut self, skill_name: &str) {
        self.skills.retain(|s| s != skill_name);
    }

    pub fn add_user(&mut self, user_name: &str) {
        if !self.users.contains(&user_name.to_string()) {
            self.users.push(user_name.to_string());
        }
    }

    pub fn remove_user(&mut self, user_name: &str) {
        self.users.retain(|u| u != user_name);
    }

    pub fn add_review(&mut self, user_name: &str, skill_name: &str, review_text: &str) {
        if self.skills.contains(&skill_name.to_string()) && self.users.contains(&user_name.to_string()) {
            self.reviews.push((user_name.to_string(), skill_name.to_string(), review_text.to_string()));
        }
    }

    pub fn get_reviews_for_skill(&self, skill_name: &str) -> Vec<(String, String)> { // (user, review)
        self.reviews.iter()
            .filter(|&&(ref u, ref s, _)| s == skill_name)
            .map(|(u, _, r)| (u.clone(), r.clone()))
            .collect()
    }

    pub fn get_reviews_for_user(&self, user_name: &str) -> Vec<(String, String)> { // (skill, review)
        self.reviews.iter()
            .filter(|&&(ref u, ref s, _)| u == user_name)
            .map(|(_, s, r)| (s.clone(), r.clone()))
            .collect()
    }
}
