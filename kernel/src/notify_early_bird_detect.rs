extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct EarlyBirdDetector {
    detected: bool,
    data: Vec<u8>,
    threshold: u8,
    results: Vec<String>,
}

impl EarlyBirdDetector {
    pub fn new(threshold: u8) -> Self {
        EarlyBirdDetector {
            detected: false,
            data: Vec::new(),
            threshold,
            results: Vec::new(),
        }
    }

    pub fn add_data(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }

    pub fn analyze(&mut self) {
        if self.data.iter().any(|&x| x > self.threshold) {
            self.detected = true;
            self.results.push(String::from("Early bird detected!"));
        } else {
            self.results.push(String::from("No early birds."));
        }
    }

    pub fn is_detected(&self) -> bool {
        self.detected
    }

    pub fn get_results(&self) -> &[String] {
        &self.results
    }

    pub fn reset(&mut self) {
        self.detected = false;
        self.data.clear();
        self.results.clear();
    }
}
