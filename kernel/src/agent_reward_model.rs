extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct AgentRewardModel {
    rewards: Vec<(String, i32)>,
}

impl AgentRewardModel {
    pub fn new() -> Self {
        AgentRewardModel {
            rewards: Vec::new(),
        }
    }

    pub fn add_reward(&mut self, action: String, reward: i32) {
        self.rewards.push((action, reward));
    }

    pub fn get_total_rewards(&self) -> i32 {
        self.rewards.iter().map(|&(_, reward)| reward).sum()
    }

    pub fn get_highest_reward(&self) -> Option<i32> {
        self.rewards.iter().map(|&(_, reward)| reward).max()
    }

    pub fn get_lowest_reward(&self) -> Option<i32> {
        self.rewards.iter().map(|&(_, reward)| reward).min()
    }

    pub fn clear_rewards(&mut self) {
        self.rewards.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_reward_model() {
        let mut model = AgentRewardModel::new();
        model.add_reward(String::from("action1"), 10);
        model.add_reward(String::from("action2"), 20);

        assert_eq!(model.get_total_rewards(), 30);
        assert_eq!(model.get_highest_reward(), Some(20));
        assert_eq!(model.get_lowest_reward(), Some(10));

        model.clear_rewards();
        assert_eq!(model.get_total_rewards(), 0);
    }
}
