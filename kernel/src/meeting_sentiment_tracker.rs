extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MeetingSentimentTracker {
    sentiments: Vec<String>,
}

impl MeetingSentimentTracker {
    pub fn new() -> Self {
        MeetingSentimentTracker {
            sentiments: Vec::new(),
        }
    }

    pub fn add_sentiment(&mut self, sentiment: String) {
        self.sentiments.push(sentiment);
    }

    pub fn get_sentiments(&self) -> &Vec<String> {
        &self.sentiments
    }

    pub fn clear_sentiments(&mut self) {
        self.sentiments.clear();
    }

    pub fn count_positive_sentiments(&self) -> usize {
        self.sentiments.iter().filter(|s| s.contains("positive")).count()
    }

    pub fn count_negative_sentiments(&self) -> usize {
        self.sentiments.iter().filter(|s| s.contains("negative")).count()
    }
}
