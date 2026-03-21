extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct AIDetector {
    known_backdoors: Vec<String>,
    detected_logs: Vec<String>,
}

impl AIDetector {
    pub fn new() -> Self {
        AIDetector {
            known_backdoors: Vec::new(),
            detected_logs: Vec::new(),
        }
    }

    pub fn add_known_backdoor(&mut self, backdoor: String) {
        self.known_backdoors.push(backdoor);
    }

    pub fn scan_process(&self, process_name: &str) -> bool {
        for backdoor in &self.known_backdoors {
            if process_name.contains(backdoor) {
                return true;
            }
        }
        false
    }

    pub fn log_detection(&mut self, process_name: String) {
        self.detected_logs.push(process_name);
    }

    pub fn get_detected_logs(&self) -> &Vec<String> {
        &self.detected_logs
    }

    pub fn clear_logs(&mut self) {
        self.detected_logs.clear();
    }
}
