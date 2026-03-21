extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct EmailUnsubscribeDetector {
    keywords: Vec<String>,
    detected_emails: Vec<String>,
}

impl EmailUnsubscribeDetector {
    pub fn new() -> Self {
        EmailUnsubscribeDetector {
            keywords: vec![
                "unsubscribe".to_string(),
                "opt-out".to_string(),
                "remove".to_string(),
                "cancel".to_string(),
                "stop".to_string(),
            ],
            detected_emails: Vec::new(),
        }
    }

    pub fn add_keyword(&mut self, keyword: String) {
        self.keywords.push(keyword);
    }

    pub fn remove_keyword(&mut self, keyword: &str) -> bool {
        if let Some(index) = self.keywords.iter().position(|k| k == keyword) {
            self.keywords.remove(index);
            true
        } else {
            false
        }
    }

    pub fn detect_unsubscribe_emails(&mut self, content: &str) {
        for line in content.lines() {
            if self.contains_keyword(line) {
                self.detected_emails.push(line.to_string());
            }
        }
    }

    pub fn get_detected_emails(&self) -> &[String] {
        &self.detected_emails
    }

    pub fn clear_detected_emails(&mut self) {
        self.detected_emails.clear();
    }

    fn contains_keyword(&self, line: &str) -> bool {
        for keyword in &self.keywords {
            if line.contains(keyword) {
                return true;
            }
        }
        false
    }
}
