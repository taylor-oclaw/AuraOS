extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeetingTopicSegmenter {
    topics: Vec<String>,
}

impl MeetingTopicSegmenter {
    pub fn new() -> Self {
        MeetingTopicSegmenter {
            topics: Vec::new(),
        }
    }

    pub fn add_topic(&mut self, topic: String) {
        self.topics.push(topic);
    }

    pub fn remove_topic(&mut self, index: usize) -> Option<String> {
        if index < self.topics.len() {
            Some(self.topics.remove(index))
        } else {
            None
        }
    }

    pub fn get_topic(&self, index: usize) -> Option<&String> {
        self.topics.get(index)
    }

    pub fn list_topics(&self) -> &[String] {
        &self.topics
    }

    pub fn clear_topics(&mut self) {
        self.topics.clear();
    }
}
