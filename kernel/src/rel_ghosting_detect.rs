extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rel_ghosting_detect_init() {
    // Initialization logic for the module
}

pub extern "C" fn rel_ghosting_detect_exit() {
    // Cleanup logic for the module
}

pub struct GhostingDetector {
    history: Vec<String>,
    threshold: usize,
}

impl GhostingDetector {
    pub fn new(threshold: usize) -> Self {
        GhostingDetector {
            history: Vec::new(),
            threshold,
        }
    }

    pub fn add_event(&mut self, event: String) {
        if self.history.len() >= self.threshold {
            self.history.remove(0);
        }
        self.history.push(event);
    }

    pub fn get_history(&self) -> &Vec<String> {
        &self.history
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    pub fn detect_ghosting(&self, event: &str) -> bool {
        self.history.iter().filter(|e| e == &&event.to_string()).count() > 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ghosting_detector_new() {
        let detector = GhostingDetector::new(5);
        assert_eq!(detector.threshold, 5);
        assert!(detector.history.is_empty());
    }

    #[test]
    fn test_ghosting_detector_add_event() {
        let mut detector = GhostingDetector::new(3);
        detector.add_event(String::from("event1"));
        detector.add_event(String::from("event2"));
        detector.add_event(String::from("event3"));
        assert_eq!(detector.history.len(), 3);

        detector.add_event(String::from("event4"));
        assert_eq!(detector.history.len(), 3);
        assert_eq!(detector.history[0], "event2");
        assert_eq!(detector.history[1], "event3");
        assert_eq!(detector.history[2], "event4");
    }

    #[test]
    fn test_ghosting_detector_get_history() {
        let mut detector = GhostingDetector::new(5);
        detector.add_event(String::from("event1"));
        detector.add_event(String::from("event2"));
        let history = detector.get_history();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0], "event1");
        assert_eq!(history[1], "event2");
    }

    #[test]
    fn test_ghosting_detector_clear_history() {
        let mut detector = GhostingDetector::new(5);
        detector.add_event(String::from("event1"));
        detector.clear_history();
        assert!(detector.history.is_empty());
    }

    #[test]
    fn test_ghosting_detector_detect_ghosting() {
        let mut detector = GhostingDetector::new(5);
        detector.add_event(String::from("event1"));
        detector.add_event(String::from("event2"));
        detector.add_event(String::from("event1"));

        assert!(detector.detect_ghosting("event1"));
        assert!(!detector.detect_ghosting("event3"));
    }
}
