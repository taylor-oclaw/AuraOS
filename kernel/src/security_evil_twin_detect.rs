extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn security_evil_twin_detect_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn security_evil_twin_detect_exit() {
    // Cleanup logic for the module
}

pub struct SecurityEvilTwinDetect {
    known_hashes: Vec<String>,
    suspicious_processes: Vec<String>,
}

impl SecurityEvilTwinDetect {
    pub fn new() -> Self {
        SecurityEvilTwinDetect {
            known_hashes: Vec::new(),
            suspicious_processes: Vec::new(),
        }
    }

    pub fn add_known_hash(&mut self, hash: &str) {
        self.known_hashes.push(hash.to_string());
    }

    pub fn is_suspicious_process(&self, process_name: &str) -> bool {
        self.suspicious_processes.contains(&process_name.to_string())
    }

    pub fn mark_as_suspicious(&mut self, process_name: &str) {
        if !self.is_suspicious_process(process_name) {
            self.suspicious_processes.push(process_name.to_string());
        }
    }

    pub fn remove_known_hash(&mut self, hash: &str) {
        self.known_hashes.retain(|h| h != hash);
    }

    pub fn check_process_integrity(&self, process_name: &str, process_hash: &str) -> bool {
        if self.is_suspicious_process(process_name) {
            return false;
        }
        self.known_hashes.contains(&process_hash.to_string())
    }
}
