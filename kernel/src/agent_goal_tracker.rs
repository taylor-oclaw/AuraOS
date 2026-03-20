extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct AgentGoalTracker {
    goals: Vec<String>,
    completed_goals: Vec<String>,
}

impl AgentGoalTracker {
    pub fn new() -> Self {
        AgentGoalTracker {
            goals: Vec::new(),
            completed_goals: Vec::new(),
        }
    }

    pub fn add_goal(&mut self, goal: String) {
        self.goals.push(goal);
    }

    pub fn mark_goal_completed(&mut self, goal: &str) -> bool {
        if let Some(index) = self.goals.iter().position(|g| g == goal) {
            let completed_goal = self.goals.remove(index);
            self.completed_goals.push(completed_goal);
            true
        } else {
            false
        }
    }

    pub fn get_active_goals(&self) -> Vec<String> {
        self.goals.clone()
    }

    pub fn get_completed_goals(&self) -> Vec<String> {
        self.completed_goals.clone()
    }

    pub fn reset_tracker(&mut self) {
        self.goals.clear();
        self.completed_goals.clear();
    }
}
