extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct EmailSentimentFlag {
    email_content: String,
    sentiment_score: i32,
}

impl EmailSentimentFlag {
    pub fn new(email_content: String) -> Self {
        EmailSentimentFlag {
            email_content,
            sentiment_score: 0,
        }
    }

    pub fn analyze_sentiment(&mut self) {
        // Simple sentiment analysis logic
        let words = self.email_content.split_whitespace();
        for word in words {
            match word.to_lowercase().as_str() {
                "good" | "great" | "excellent" => self.sentiment_score += 1,
                "bad" | "terrible" | "poor" => self.sentiment_score -= 1,
                _ => {}
            }
        }
    }

    pub fn get_sentiment_score(&self) -> i32 {
        self.sentiment_score
    }

    pub fn is_positive(&self) -> bool {
        self.sentiment_score > 0
    }

    pub fn is_negative(&self) -> bool {
        self.sentiment_score < 0
    }

    pub fn reset_sentiment(&mut self) {
        self.sentiment_score = 0;
    }
}
