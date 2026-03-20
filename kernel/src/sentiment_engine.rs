extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub enum Sentiment {
    Positive,
    Negative,
    Neutral,
    Mixed,
}

pub struct SentimentResult {
    pub text: String,
    pub sentiment: Sentiment,
    pub confidence: f32,
    pub agent_id: u64,
    pub timestamp: u64,
}

pub struct SentimentEngine {
    pub results: Vec<SentimentResult>,
    pub positive_words: Vec<String>,
    pub negative_words: Vec<String>,
}

impl SentimentEngine {
    pub fn new() -> Self {
        let pos = vec![
            "good", "great", "excellent", "happy", "love", "success", "wonderful", "amazing",
            "perfect", "best",
        ]
        .iter()
        .map(|w| String::from(*w))
        .collect();
        let neg = vec![
            "bad", "terrible", "awful", "hate", "fail", "error", "worst", "broken", "crash", "slow",
        ]
        .iter()
        .map(|w| String::from(*w))
        .collect();
        Self {
            results: Vec::new(),
            positive_words: pos,
            negative_words: neg,
        }
    }

    pub fn analyze(&mut self, text: &str, agent_id: u64) -> Sentiment {
        let lower = text.to_lowercase();
        let pos_count = self
            .positive_words
            .iter()
            .filter(|w| lower.contains(w.as_str()))
            .count();
        let neg_count = self
            .negative_words
            .iter()
            .filter(|w| lower.contains(w.as_str()))
            .count();

        let sentiment = if pos_count > 0 && neg_count > 0 {
            Sentiment::Mixed
        } else if pos_count > neg_count {
            Sentiment::Positive
        } else if neg_count > pos_count {
            Sentiment::Negative
        } else {
            Sentiment::Neutral
        };

        let total = (pos_count + neg_count) as f32;
        let confidence = if total > 0.0 {
            (pos_count.max(neg_count) as f32) / total
        } else {
            0.5
        };

        let is_positive = matches!(sentiment, Sentiment::Positive);
        let is_negative = matches!(sentiment, Sentiment::Negative);
        let is_mixed = matches!(sentiment, Sentiment::Mixed);

        self.results.push(SentimentResult {
            text: String::from(text),
            sentiment,
            confidence,
            agent_id,
            timestamp: 0,
        });

        if is_mixed { Sentiment::Mixed }
        else if is_positive { Sentiment::Positive }
        else if is_negative { Sentiment::Negative }
        else { Sentiment::Neutral }
    }

    pub fn positive_ratio(&self) -> f32 {
        if self.results.is_empty() {
            0.0
        } else {
            self.results
                .iter()
                .filter(|r| matches!(r.sentiment, Sentiment::Positive))
                .count() as f32
                / self.results.len() as f32
        }
    }

    pub fn total(&self) -> usize {
        self.results.len()
    }
}
