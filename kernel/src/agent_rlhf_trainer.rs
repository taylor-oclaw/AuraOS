extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct AgentRLHFTrainer {
    name: String,
    episodes: Vec<Episode>,
    learning_rate: f64,
    discount_factor: f64,
}

impl AgentRLHFTrainer {
    pub fn new(name: &str, learning_rate: f64, discount_factor: f64) -> Self {
        AgentRLHFTrainer {
            name: String::from(name),
            episodes: Vec::new(),
            learning_rate,
            discount_factor,
        }
    }

    pub fn add_episode(&mut self, episode: Episode) {
        self.episodes.push(episode);
    }

    pub fn get_episodes_count(&self) -> usize {
        self.episodes.len()
    }

    pub fn calculate_total_reward(&self) -> f64 {
        self.episodes.iter().map(|e| e.total_reward()).sum()
    }

    pub fn update_learning_rate(&mut self, new_rate: f64) {
        self.learning_rate = new_rate;
    }
}

pub struct Episode {
    steps: Vec<Step>,
}

impl Episode {
    pub fn new() -> Self {
        Episode { steps: Vec::new() }
    }

    pub fn add_step(&mut self, step: Step) {
        self.steps.push(step);
    }

    pub fn total_reward(&self) -> f64 {
        self.steps.iter().map(|s| s.reward).sum()
    }
}

pub struct Step {
    action: String,
    reward: f64,
}

impl Step {
    pub fn new(action: &str, reward: f64) -> Self {
        Step {
            action: String::from(action),
            reward,
        }
    }
}
