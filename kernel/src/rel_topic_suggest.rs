extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct TopicSuggester {
    topics: Vec<String>,
}

impl TopicSuggester {
    pub fn new() -> Self {
        TopicSuggester {
            topics: Vec::new(),
        }
    }

    pub fn add_topic(&mut self, topic: String) {
        if !self.topics.contains(&topic) {
            self.topics.push(topic);
        }
    }

    pub fn remove_topic(&mut self, topic: &str) -> bool {
        let pos = self.topics.iter().position(|t| t == topic);
        if let Some(index) = pos {
            self.topics.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_all_topics(&self) -> Vec<String> {
        self.topics.clone()
    }

    pub fn suggest_topic(&self, keyword: &str) -> Option<String> {
        for topic in &self.topics {
            if topic.contains(keyword) {
                return Some(topic.clone());
            }
        }
        None
    }

    pub fn count_topics(&self) -> usize {
        self.topics.len()
    }
}
