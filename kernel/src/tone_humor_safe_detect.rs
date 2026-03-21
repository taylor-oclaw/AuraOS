extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

mod tone_humor_safe_detect {
    use super::*;

    pub struct ToneHumorSafeDetect {
        // Example fields for the detector
        keywords: Vec<String>,
        safe_threshold: u32,
    }

    impl ToneHumorSafeDetect {
        pub fn new(keywords: Vec<String>, safe_threshold: u32) -> Self {
            ToneHumorSafeDetect {
                keywords,
                safe_threshold,
            }
        }

        pub fn add_keyword(&mut self, keyword: String) {
            self.keywords.push(keyword);
        }

        pub fn remove_keyword(&mut self, keyword: &str) {
            if let Some(pos) = self.keywords.iter().position(|k| k == keyword) {
                self.keywords.remove(pos);
            }
        }

        pub fn is_safe(&self, text: &str) -> bool {
            let mut count = 0;
            for keyword in &self.keywords {
                if text.contains(keyword) {
                    count += 1;
                }
            }
            count < self.safe_threshold
        }

        pub fn get_keywords_count(&self) -> usize {
            self.keywords.len()
        }

        pub fn set_safe_threshold(&mut self, threshold: u32) {
            self.safe_threshold = threshold;
        }
    }
}

// Example usage within the kernel module context
fn main() {
    let mut detector = tone_humor_safe_detect::ToneHumorSafeDetect::new(
        vec![String::from("badword1"), String::from("badword2")],
        1,
    ;

    detector.add_keyword(String::from("badword3"));
    assert!(detector.is_safe("This is a safe text."));
    assert!(!detector.is_safe("This contains badword1 and badword2."));

    detector.remove_keyword("badword1");
    assert!(detector.is_safe("This contains badword2."));

    detector.set_safe_threshold(2);
}
