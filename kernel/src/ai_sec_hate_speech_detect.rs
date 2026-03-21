extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct HateSpeechDetector {
    keywords: Vec<String>,
}

impl HateSpeechDetector {
    pub fn new(keywords: Vec<&str>) -> Self {
        HateSpeechDetector {
            keywords: keywords.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn add_keyword(&mut self, keyword: &str) {
        if !self.keywords.contains(&keyword.to_string()) {
            self.keywords.push(keyword.to_string());
        }
    }

    pub fn remove_keyword(&mut self, keyword: &str) {
        self.keywords.retain(|k| k != keyword);
    }

    pub fn contains_hate_speech(&self, text: &str) -> bool {
        for keyword in &self.keywords {
            if text.contains(keyword) {
                return true;
            }
        }
        false
    }

    pub fn list_keywords(&self) -> Vec<String> {
        self.keywords.clone()
    }

    pub fn clear_keywords(&mut self) {
        self.keywords.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hate_speech_detector() {
        let mut detector = HateSpeechDetector::new(vec!["hate", "abuse"]);
        assert_eq!(detector.contains_hate_speech("This is a hate speech"), true);
        assert_eq!(detector.contains_hate_speech("This is clean text"), false);

        detector.add_keyword("offensive");
        assert_eq!(detector.contains_hate_speech("This is offensive language"), true);

        detector.remove_keyword("hate");
        assert_eq!(detector.contains_hate_speech("This is a hate speech"), false);

        let keywords = detector.list_keywords();
        assert_eq!(keywords, vec!["abuse", "offensive"]);

        detector.clear_keywords();
        assert_eq!(detector.contains_hate_speech("This is offensive language"), false);
    }
}
