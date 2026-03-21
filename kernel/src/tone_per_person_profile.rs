extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn tone_per_person_profile_init() {
    // Initialization logic for the module
}

pub extern "C" fn tone_per_person_profile_exit() {
    // Cleanup logic for the module
}

pub struct TonePerPersonProfile {
    name: String,
    age: u32,
    interests: Vec<String>,
    mood_scores: Vec<f32>,
    last_interaction_time: u64,
}

impl TonePerPersonProfile {
    pub fn new(name: &str, age: u32) -> Self {
        TonePerPersonProfile {
            name: String::from(name),
            age,
            interests: Vec::new(),
            mood_scores: Vec::new(),
            last_interaction_time: 0,
        }
    }

    pub fn add_interest(&mut self, interest: &str) {
        self.interests.push(String::from(interest));
    }

    pub fn record_mood_score(&mut self, score: f32) {
        self.mood_scores.push(score);
    }

    pub fn update_last_interaction_time(&mut self, time: u64) {
        self.last_interaction_time = time;
    }

    pub fn get_average_mood_score(&self) -> Option<f32> {
        if self.mood_scores.is_empty() {
            None
        } else {
            let sum: f32 = self.mood_scores.iter().sum();
            Some(sum / self.mood_scores.len() as f32)
        }
    }

    pub fn get_last_interaction_time(&self) -> u64 {
        self.last_interaction_time
    }
}
