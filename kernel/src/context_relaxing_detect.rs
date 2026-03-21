extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ContextRelaxingDetect {
    detected_events: Vec<String>,
    threshold: usize,
}

impl ContextRelaxingDetect {
    pub fn new(threshold: usize) -> Self {
        ContextRelaxingDetect {
            detected_events: Vec::new(),
            threshold,
        }
    }

    pub fn add_event(&mut self, event: String) {
        if self.detected_events.len() >= self.threshold {
            self.detected_events.remove(0);
        }
        self.detected_events.push(event);
    }

    pub fn get_detected_events(&self) -> &Vec<String> {
        &self.detected_events
    }

    pub fn clear_events(&mut self) {
        self.detected_events.clear();
    }

    pub fn is_threshold_reached(&self) -> bool {
        self.detected_events.len() >= self.threshold
    }

    pub fn get_event_count(&self) -> usize {
        self.detected_events.len()
    }
}
