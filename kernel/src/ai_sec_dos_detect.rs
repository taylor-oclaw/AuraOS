extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct AIDoSModule {
    detected_attacks: Vec<String>,
    threshold: usize,
}

impl AIDoSModule {
    pub fn new(threshold: usize) -> Self {
        AIDoSModule {
            detected_attacks: Vec::new(),
            threshold,
        }
    }

    pub fn log_attack(&mut self, attack_description: &str) {
        if self.detected_attacks.len() < self.threshold {
            self.detected_attacks.push(String::from(attack_description));
        }
    }

    pub fn get_detected_attacks(&self) -> &[String] {
        &self.detected_attacks
    }

    pub fn clear_detected_attacks(&mut self) {
        self.detected_attacks.clear();
    }

    pub fn is_threshold_exceeded(&self) -> bool {
        self.detected_attacks.len() >= self.threshold
    }
}
