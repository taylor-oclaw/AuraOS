extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct ConversationStarter {
    topics: Vec<String>,
}

impl ConversationStarter {
    pub fn new() -> Self {
        ConversationStarter {
            topics: vec![
                String::from("AI ethics"),
                String::from("Quantum computing"),
                String::from("Blockchain technology"),
                String::from("Cybersecurity"),
                String::from("Robotics and automation"),
            ],
        }
    }

    pub fn add_topic(&mut self, topic: &str) {
        self.topics.push(String::from(topic));
    }

    pub fn remove_topic(&mut self, topic: &str) -> bool {
        if let Some(index) = self.topics.iter().position(|t| t == topic) {
            self.topics.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_topics(&self) -> Vec<String> {
        self.topics.clone()
    }

    pub fn get_random_topic(&self) -> Option<&String> {
        if self.topics.is_empty() {
            None
        } else {
            Some(&self.topics[0]) // Simplified for no_std compatibility
        }
    }

    pub fn has_topic(&self, topic: &str) -> bool {
        self.topics.contains(&String::from(topic))
    }
}
