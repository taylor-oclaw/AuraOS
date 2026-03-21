extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct EmailSpamAI {
    // Example fields for the AI model
    weights: Vec<f32>,
    biases: Vec<f32>,
    vocabulary: Vec<String>,
}

impl EmailSpamAI {
    pub fn new() -> Self {
        // Initialize with dummy data, replace with actual training logic
        EmailSpamAI {
            weights: vec![0.1, 0.2, 0.3],
            biases: vec![0.05, 0.1],
            vocabulary: vec![
                String::from("free"),
                String::from("win"),
                String::from("prize"),
            ],
        }
    }

    pub fn train(&mut self, data: Vec<(String, bool)>) {
        // Dummy training logic
        for (email, is_spam) in data {
            if is_spam {
                self.weights.iter_mut().for_each(|w| *w += 0.01);
            } else {
                self.weights.iter_mut().for_each(|w| *w -= 0.01);
            }
        }
    }

    pub fn predict(&self, email: &str) -> bool {
        // Dummy prediction logic
        let features = self.extract_features(email);
        let score = features.iter()
                           .zip(self.weights.iter())
                           .map(|(f, w)| f * w)
                           .sum::<f32>() + self.biases[0];
        score > 0.5
    }

    fn extract_features(&self, email: &str) -> Vec<f32> {
        // Dummy feature extraction logic
        let mut features = vec![0.0; self.weights.len()];
        for (i, word) in self.vocabulary.iter().enumerate() {
            if email.contains(word) {
                features[i] = 1.0;
            }
        }
        features
    }

    pub fn update_vocabulary(&mut self, new_words: Vec<String>) {
        // Add new words to the vocabulary
        self.vocabulary.extend(new_words);
        self.weights.push(0.0); // Initialize with a default weight
    }

    pub fn get_vocabulary_size(&self) -> usize {
        // Return the size of the vocabulary
        self.vocabulary.len()
    }
}
