extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct TextPreferenceDetector {
    preferences: Vec<String>,
}

impl TextPreferenceDetector {
    pub fn new() -> Self {
        TextPreferenceDetector {
            preferences: Vec::new(),
        }
    }

    pub fn add_preference(&mut self, preference: String) {
        self.preferences.push(preference);
    }

    pub fn remove_preference(&mut self, preference: &str) -> bool {
        if let Some(index) = self.preferences.iter().position(|p| p == preference) {
            self.preferences.remove(index);
            true
        } else {
            false
        }
    }

    pub fn has_preference(&self, preference: &str) -> bool {
        self.preferences.contains(&String::from(preference))
    }

    pub fn list_preferences(&self) -> Vec<String> {
        self.preferences.clone()
    }

    pub fn clear_preferences(&mut self) {
        self.preferences.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_preference() {
        let mut detector = TextPreferenceDetector::new();
        detector.add_preference(String::from("AI"));
        assert_eq!(detector.list_preferences(), vec![String::from("AI")]);
    }

    #[test]
    fn test_remove_preference() {
        let mut detector = TextPreferenceDetector::new();
        detector.add_preference(String::from("AI"));
        assert!(detector.remove_preference("AI"));
        assert_eq!(detector.list_preferences().len(), 0);
    }

    #[test]
    fn test_has_preference() {
        let mut detector = TextPreferenceDetector::new();
        detector.add_preference(String::from("AI"));
        assert!(detector.has_preference("AI"));
        assert!(!detector.has_preference("Machine Learning"));
    }

    #[test]
    fn test_list_preferences() {
        let mut detector = TextPreferenceDetector::new();
        detector.add_preference(String::from("AI"));
        detector.add_preference(String::from("Machine Learning"));
        assert_eq!(
            detector.list_preferences(),
            vec![String::from("AI"), String::from("Machine Learning")]
        );
    }

    #[test]
    fn test_clear_preferences() {
        let mut detector = TextPreferenceDetector::new();
        detector.add_preference(String::from("AI"));
        detector.clear_preferences();
        assert_eq!(detector.list_preferences().len(), 0);
    }
}
