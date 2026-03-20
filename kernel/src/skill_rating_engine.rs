extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

mod skill_rating_engine {
    use super::*;

    pub struct SkillRatingEngine {
        ratings: Vec<(String, u32)>,
    }

    impl SkillRatingEngine {
        pub fn new() -> Self {
            SkillRatingEngine { ratings: Vec::new() }
        }

        pub fn add_skill(&mut self, skill_name: String, rating: u32) {
            self.ratings.push((skill_name, rating));
        }

        pub fn get_rating(&self, skill_name: &str) -> Option<u32> {
            self.ratings.iter().find(|&&(ref name, _)| name == skill_name).map(|&(_, rating)| rating)
        }

        pub fn update_rating(&mut self, skill_name: &str, new_rating: u32) -> bool {
            if let Some(index) = self.ratings.iter().position(|&(ref name, _)| name == skill_name) {
                self.ratings[index].1 = new_rating;
                true
            } else {
                false
            }
        }

        pub fn remove_skill(&mut self, skill_name: &str) -> bool {
            if let Some(index) = self.ratings.iter().position(|&(ref name, _)| name == skill_name) {
                self.ratings.remove(index);
                true
            } else {
                false
            }
        }

        pub fn list_skills(&self) -> Vec<(String, u32)> {
            self.ratings.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::skill_rating_engine::*;

    #[test]
    fn test_skill_rating_engine() {
        let mut engine = SkillRatingEngine::new();
        assert_eq!(engine.list_skills(), vec![]);

        engine.add_skill(String::from("Rust"), 85);
        engine.add_skill(String::from("C++"), 90);
        assert_eq!(engine.get_rating("Rust"), Some(85));
        assert_eq!(engine.get_rating("C++"), Some(90));

        assert!(engine.update_rating("Rust", 95));
        assert_eq!(engine.get_rating("Rust"), Some(95));

        assert!(engine.remove_skill("C++"));
        assert_eq!(engine.get_rating("C++"), None);

        let skills = engine.list_skills();
        assert_eq!(skills.len(), 1);
        assert_eq!(skills[0], (String::from("Rust"), 95));
    }
}
