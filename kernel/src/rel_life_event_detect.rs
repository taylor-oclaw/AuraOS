extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut detector = LifeEventDetector::new();
    detector.log_event("System Booted");
    detector.register_interest(String::from("Shutdown"));
    detector.log_event("User Login");
    detector.log_event("System Shutdown Initiated");
    detector.check_interests();
}

pub struct LifeEventDetector {
    events: Vec<String>,
    interests: Vec<String>,
}

impl LifeEventDetector {
    pub fn new() -> Self {
        LifeEventDetector {
            events: Vec::new(),
            interests: Vec::new(),
        }
    }

    pub fn log_event(&mut self, event: &str) {
        self.events.push(String::from(event));
    }

    pub fn register_interest(&mut self, interest: String) {
        self.interests.push(interest);
    }

    pub fn get_events(&self) -> &[String] {
        &self.events
    }

    pub fn get_interests(&self) -> &[String] {
        &self.interests
    }

    pub fn check_interests(&self) {
        for event in &self.events {
            if self.interests.contains(event) {
                // Handle the interested event, e.g., trigger a callback or log it
                println!("Interest met: {}", event);
            }
        }
    }
}
