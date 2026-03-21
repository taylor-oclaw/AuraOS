extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct BehaviorShiftDetector {
    user_id: u32,
    behavior_history: Vec<String>,
    threshold: f32,
}

impl BehaviorShiftDetector {
    pub fn new(user_id: u32, threshold: f32) -> Self {
        BehaviorShiftDetector {
            user_id,
            behavior_history: Vec::new(),
            threshold,
        }
    }

    pub fn add_behavior(&mut self, behavior: String) {
        self.behavior_history.push(behavior);
    }

    pub fn get_user_id(&self) -> u32 {
        self.user_id
    }

    pub fn calculate_shift_score(&self) -> f32 {
        if self.behavior_history.len() < 2 {
            return 0.0;
        }
        let mut score = 0.0;
        for i in 1..self.behavior_history.len() {
            // Simple heuristic: compare the last two behaviors
            if self.behavior_history[i] != self.behavior_history[i - 1] {
                score += 1.0;
            }
        }
        score / (self.behavior_history.len() as f32 - 1.0)
    }

    pub fn is_behavior_shifted(&self) -> bool {
        let shift_score = self.calculate_shift_score();
        shift_score > self.threshold
    }

    pub fn clear_history(&mut self) {
        self.behavior_history.clear();
    }
}
