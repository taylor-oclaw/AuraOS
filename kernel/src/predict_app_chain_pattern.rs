extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct PredictAppChainPattern {
    patterns: Vec<String>,
}

impl PredictAppChainPattern {
    pub fn new() -> Self {
        PredictAppChainPattern {
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

    pub fn get_pattern(&self, index: usize) -> Option<&String> {
        self.patterns.get(index)
    }

    pub fn list_patterns(&self) -> Vec<String> {
        self.patterns.clone()
    }

    pub fn clear_patterns(&mut self) {
        self.patterns.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict_app_chain_pattern() {
        let mut pattern = PredictAppChainPattern::new();
        assert_eq!(pattern.list_patterns(), Vec::<String>::new());

        pattern.add_pattern(String::from("pattern1"));
        pattern.add_pattern(String::from("pattern2"));
        assert_eq!(pattern.list_patterns().len(), 2);

        assert_eq!(pattern.get_pattern(0), Some(&String::from("pattern1")));
        assert_eq!(pattern.remove_pattern(0), Some(String::from("pattern1")));
        assert_eq!(pattern.list_patterns().len(), 1);

        pattern.clear_patterns();
        assert_eq!(pattern.list_patterns(), Vec::<String>::new());
    }
}
