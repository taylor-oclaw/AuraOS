extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct FinanceGoalTracker {
    goals: Vec<Goal>,
}

impl FinanceGoalTracker {
    pub fn new() -> Self {
        FinanceGoalTracker { goals: Vec::new() }
    }

    pub fn add_goal(&mut self, name: String, amount: u32) {
        let goal = Goal { name, amount, saved: 0 };
        self.goals.push(goal);
    }

    pub fn save_amount(&mut self, goal_name: &str, amount: u32) -> Result<(), &'static str> {
        for goal in self.goals.iter_mut() {
            if goal.name == goal_name {
                goal.saved += amount;
                return Ok(());
            }
        }
        Err("Goal not found")
    }

    pub fn get_goal_progress(&self, goal_name: &str) -> Result<(u32, u32), &'static str> {
        for goal in self.goals.iter() {
            if goal.name == goal_name {
                return Ok((goal.saved, goal.amount));
            }
        }
        Err("Goal not found")
    }

    pub fn list_goals(&self) -> Vec<String> {
        self.goals.iter().map(|g| g.name.clone()).collect()
    }

    pub fn remove_goal(&mut self, goal_name: &str) -> Result<(), &'static str> {
        let pos = self.goals.iter().position(|g| g.name == goal_name);
        match pos {
            Some(index) => {
                self.goals.remove(index);
                Ok(())
            }
            None => Err("Goal not found"),
        }
    }
}

#[derive(Debug)]
struct Goal {
    name: String,
    amount: u32,
    saved: u32,
}
