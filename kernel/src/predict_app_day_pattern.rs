extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct DayPatternPredictor {
    patterns: Vec<String>,
}

impl DayPatternPredictor {
    pub fn new() -> Self {
        DayPatternPredictor {
            patterns: Vec::new(),
        }
    }

    pub fn add_pattern(&mut self, pattern: String) {
        self.patterns.push(pattern);
    }

    pub fn remove_pattern(&mut self, index: usize) -> Option<String> {
        if index < self.patterns.len() {
            Some(self.patterns.remove(index))
        } else {
            None
        }
    }

    pub fn get_patterns(&self) -> Vec<String> {
        self.patterns.clone()
    }

    pub fn predict_next_day(&self) -> Option<&String> {
        if !self.patterns.is_empty() {
            Some(&self.patterns[self.patterns.len() % self.patterns.len()])
        } else {
            None
        }
    }

    pub fn clear_patterns(&mut self) {
        self.patterns.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_pattern_predictor() {
        let mut predictor = DayPatternPredictor::new();
        assert_eq!(predictor.get_patterns().len(), 0);

        predictor.add_pattern(String::from("Work"));
        predictor.add_pattern(String::from("Rest"));

        assert_eq!(predictor.get_patterns().len(), 2);
        assert_eq!(predictor.predict_next_day(), Some(&String::from("Work")));

        let removed = predictor.remove_pattern(0);
        assert_eq!(removed, Some(String::from("Work")));
        assert_eq!(predictor.get_patterns().len(), 1);

        predictor.clear_patterns();
        assert_eq!(predictor.get_patterns().len(), 0);
    }
}
