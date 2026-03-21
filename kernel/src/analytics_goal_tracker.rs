extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AnalyticsGoalTracker {
    goals: Vec<String>,
    completed_goals: usize,
}

impl AnalyticsGoalTracker {
    pub fn new() -> Self {
        AnalyticsGoalTracker {
            goals: Vec::new(),
            completed_goals: 0,
        }
    }

    pub fn add_goal(&mut self, goal: String) {
        self.goals.push(goal);
    }

    pub fn complete_goal(&mut self, index: usize) -> Result<(), &'static str> {
        if index < self.goals.len() {
            self.completed_goals += 1;
            Ok(())
        } else {
            Err("Goal index out of bounds")
        }
    }

    pub fn get_total_goals(&self) -> usize {
        self.goals.len()
    }

    pub fn get_completed_goals(&self) -> usize {
        self.completed_goals
    }

    pub fn list_goals(&self) -> Vec<String> {
        self.goals.clone()
    }
}
