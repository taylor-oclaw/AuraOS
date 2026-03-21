extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn security_keylogger_detect_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn security_keylogger_detect_exit() {
    // Cleanup logic for the module
}

pub struct SecurityKeyloggerDetect {
    detected_keys: Vec<String>,
    threshold: usize,
}

impl SecurityKeyloggerDetect {
    pub fn new(threshold: usize) -> Self {
        SecurityKeyloggerDetect {
            detected_keys: Vec::new(),
            threshold,
        }
    }

    pub fn add_detected_key(&mut self, key: String) {
        if self.detected_keys.len() >= self.threshold {
            self.detected_keys.remove(0);
        }
        self.detected_keys.push(key);
    }

    pub fn get_detected_keys(&self) -> &Vec<String> {
        &self.detected_keys
    }

    pub fn clear_detected_keys(&mut self) {
        self.detected_keys.clear();
    }

    pub fn is_keylogger_present(&self) -> bool {
        self.detected_keys.len() >= self.threshold
    }
}
