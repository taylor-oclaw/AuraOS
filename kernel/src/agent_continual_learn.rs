extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct AgentContinualLearn {
    knowledge_base: Vec<String>,
    learning_rate: f32,
    experience_count: usize,
}

impl AgentContinualLearn {
    pub fn new(learning_rate: f32) -> Self {
        AgentContinualLearn {
            knowledge_base: Vec::new(),
            learning_rate,
            experience_count: 0,
        }
    }

    pub fn add_experience(&mut self, experience: String) {
        self.knowledge_base.push(experience);
        self.experience_count += 1;
    }

    pub fn get_knowledge_base_size(&self) -> usize {
        self.knowledge_base.len()
    }

    pub fn update_learning_rate(&mut self, new_rate: f32) {
        self.learning_rate = new_rate;
    }

    pub fn get_learning_rate(&self) -> f32 {
        self.learning_rate
    }

    pub fn get_experience_count(&self) -> usize {
        self.experience_count
    }
}
