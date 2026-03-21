extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct RelMoodDetectText {
    // Example fields for the mood detection module
    pub text_data: Vec<String>,
    pub positive_keywords: Vec<String>,
    pub negative_keywords: Vec<String>,
}

impl RelMoodDetectText {
    // Constructor to initialize the mood detection module
    pub fn new() -> Self {
        RelMoodDetectText {
            text_data: Vec::new(),
            positive_keywords: vec![
                "happy".to_string(),
                "joyful".to_string(),
                "excited".to_string(),
            ],
            negative_keywords: vec![
                "sad".to_string(),
                "unhappy".to_string(),
                "angry".to_string(),
            ],
        }
    }

    // Method to add text data for analysis
    pub fn add_text(&mut self, text: String) {
        self.text_data.push(text);
    }

    // Method to analyze the mood of the stored text data
    pub fn analyze_mood(&self) -> String {
        let mut positive_count = 0;
        let mut negative_count = 0;

        for text in &self.text_data {
            if self.contains_keywords(text, &self.positive_keywords) {
                positive_count += 1;
            }
            if self.contains_keywords(text, &self.negative_keywords) {
                negative_count += 1;
            }
        }

        if positive_count > negative_count {
            "Positive".to_string()
        } else if negative_count > positive_count {
            "Negative".to_string()
        } else {
            "Neutral".to_string()
        }
    }

    // Helper method to check for keywords in a given text
    fn contains_keywords(&self, text: &str, keywords: &[String]) -> bool {
        for keyword in keywords {
            if text.contains(keyword) {
                return true;
            }
        }
        false
    }

    // Method to clear all stored text data
    pub fn clear_text_data(&mut self) {
        self.text_data.clear();
    }

    // Method to get the count of positive and negative keywords found in the text data
    pub fn keyword_counts(&self) -> (usize, usize) {
        let mut positive_count = 0;
        let mut negative_count = 0;

        for text in &self.text_data {
            if self.contains_keywords(text, &self.positive_keywords) {
                positive_count += 1;
            }
            if self.contains_keywords(text, &self.negative_keywords) {
                negative_count += 1;
            }
        }

        (positive_count, negative_count)
    }
}
