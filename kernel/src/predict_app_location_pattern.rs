extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_std]
mod predict_app_location_pattern {
    extern crate alloc;
    use alloc::string::String;
    use alloc::vec::Vec;

    pub struct AppLocationPredictor {
        patterns: Vec<String>,
        weights: Vec<f32>,
    }

    impl AppLocationPredictor {
        pub fn new(patterns: Vec<String>, weights: Vec<f32>) -> Self {
            assert_eq!(patterns.len(), weights.len());
            AppLocationPredictor { patterns, weights }
        }

        pub fn add_pattern(&mut self, pattern: String, weight: f32) {
            self.patterns.push(pattern);
            self.weights.push(weight);
        }

        pub fn remove_pattern(&mut self, index: usize) {
            if index < self.patterns.len() {
                self.patterns.remove(index);
                self.weights.remove(index);
            }
        }

        pub fn get_patterns(&self) -> &Vec<String> {
            &self.patterns
        }

        pub fn predict_location(&self, input: &str) -> Option<&String> {
            let mut max_score = -1.0;
            let mut predicted_pattern: Option<&String> = None;

            for (pattern, weight) in self.patterns.iter().zip(self.weights.iter()) {
                let score = self.calculate_similarity(input, pattern);
                if score > max_score {
                    max_score = score;
                    predicted_pattern = Some(pattern);
                }
            }

            predicted_pattern
        }

        fn calculate_similarity(&self, input: &str, pattern: &String) -> f32 {
            // Simple similarity calculation (e.g., Levenshtein distance)
            let mut distance = 0;
            let input_chars: Vec<char> = input.chars().collect();
            let pattern_chars: Vec<char> = pattern.chars().collect();

            for i in 0..input_chars.len() {
                if i >= pattern_chars.len() || input_chars[i] != pattern_chars[i] {
                    distance += 1;
                }
            }

            // Normalize the distance to a similarity score
            let max_len = input_chars.len().max(pattern_chars.len()) as f32;
            if max_len == 0.0 {
                return 1.0; // Both strings are empty
            }
            1.0 - (distance as f32 / max_len)
        }
    }
}
