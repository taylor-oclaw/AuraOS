extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct CyberbullyDetector {
    keywords: Vec<String>,
    detected_messages: Vec<String>,
}

impl CyberbullyDetector {
    pub fn new(keywords: Vec<&str>) -> Self {
        CyberbullyDetector {
            keywords: keywords.into_iter().map(|s| s.to_string()).collect(),
            detected_messages: Vec::new(),
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

    pub fn scan_message(&mut self, message: &str) -> bool {
        let is_cyberbully = self.keywords.iter().any(|keyword| message.contains(keyword));
        if is_cyberbully {
            self.detected_messages.push(message.to_string());
        }
        is_cyberbully
    }

    pub fn get_detected_messages(&self) -> Vec<String> {
        self.detected_messages.clone()
    }

    pub fn clear_detected_messages(&mut self) {
        self.detected_messages.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cyberbully_detector() {
        let mut detector = CyberbullyDetector::new(vec!["mean", "hate"]);
        assert_eq!(detector.scan_message("This is a mean message"), true);
        assert_eq!(detector.scan_message("This is a nice message"), false);
        assert_eq!(detector.get_detected_messages(), vec![String::from("This is a mean message")]);
        detector.clear_detected_messages();
        assert_eq!(detector.get_detected_messages().len(), 0);
    }
}
