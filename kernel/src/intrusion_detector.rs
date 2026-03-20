extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    let detector = IntrusionDetector::new();
    detector.log_event("System Booted");
    detector.add_whitelist_pattern("safe_process");
    detector.check_activity("safe_process");
    detector.check_activity("malicious_process");
    detector.list_events();
}

pub struct IntrusionDetector {
    events: Vec<String>,
    whitelist_patterns: Vec<String>,
}

impl IntrusionDetector {
    pub fn new() -> Self {
        IntrusionDetector {
            events: Vec::new(),
            whitelist_patterns: Vec::new(),
        }
    }

    pub fn log_event(&mut self, event: &str) {
        self.events.push(event.to_string());
    }

    pub fn add_whitelist_pattern(&mut self, pattern: &str) {
        self.whitelist_patterns.push(pattern.to_string());
    }

    pub fn check_activity(&self, activity: &str) -> bool {
        if self.whitelist_patterns.iter().any(|p| activity.contains(p)) {
            return true;
        }
        self.log_event(&String::from("info"));
        false
    }

    pub fn list_events(&self) {
        for event in &self.events {
            // Simulate logging or outputting events
        }
    }

    pub fn clear_events(&mut self) {
        self.events.clear();
    }
}
