extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct ToneSlangMatch {
    patterns: Vec<String>,
}

impl ToneSlangMatch {
    pub fn new(patterns: Vec<&str>) -> Self {
        let patterns = patterns.into_iter().map(|p| p.to_string()).collect();
        ToneSlangMatch { patterns }
    }

    pub fn add_pattern(&mut self, pattern: &str) {
        self.patterns.push(pattern.to_string());
    }

    pub fn remove_pattern(&mut self, pattern: &str) -> bool {
        if let Some(index) = self.patterns.iter().position(|p| p == pattern) {
            self.patterns.remove(index);
            true
        } else {
            false
        }
    }

    pub fn match_text(&self, text: &str) -> Vec<String> {
        self.patterns
            .iter()
            .filter(|pattern| text.contains(pattern))
            .cloned()
            .collect()
    }

    pub fn count_patterns(&self) -> usize {
        self.patterns.len()
    }

    pub fn list_patterns(&self) -> Vec<String> {
        self.patterns.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tone_slang_match() {
        let mut matcher = ToneSlangMatch::new(vec!["hello", "world"]);
        assert_eq!(matcher.match_text("hello there"), vec![String::from("hello")]);
        assert_eq!(matcher.count_patterns(), 2);
        assert!(matcher.remove_pattern("hello"));
        assert_eq!(matcher.count_patterns(), 1);
        matcher.add_pattern("rust");
        assert_eq!(matcher.list_patterns(), vec![String::from("world"), String::from("rust")]);
    }
}
