extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct SelfHarmDetection {
    keywords: Vec<String>,
    detected: bool,
}

impl SelfHarmDetection {
    pub fn new(keywords: Vec<&str>) -> SelfHarmDetection {
        let keyword_strings = keywords.into_iter().map(|s| String::from(s)).collect();
        SelfHarmDetection {
            keywords: keyword_strings,
            detected: false,
        }
    }

    pub fn add_keyword(&mut self, keyword: &str) {
        self.keywords.push(String::from(keyword));
    }

    pub fn remove_keyword(&mut self, keyword: &str) {
        self.keywords.retain(|k| k != keyword);
    }

    pub fn check_text(&mut self, text: &str) -> bool {
        for keyword in &self.keywords {
            if text.contains(keyword) {
                self.detected = true;
                return true;
            }
        }
        false
    }

    pub fn reset_detection(&mut self) {
        self.detected = false;
    }

    pub fn is_detected(&self) -> bool {
        self.detected
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_harm_detection() {
        let mut detector = SelfHarmDetection::new(vec!["hurt", "kill"]);
        assert_eq!(detector.check_text("I want to hurt someone"), true);
        assert_eq!(detector.is_detected(), true);
        detector.reset_detection();
        assert_eq!(detector.is_detected(), false);
        detector.add_keyword("suicide");
        assert_eq!(detector.check_text("I am thinking about suicide"), true);
        detector.remove_keyword("hurt");
        assert_eq!(detector.check_text("I want to hurt someone"), false);
    }
}
