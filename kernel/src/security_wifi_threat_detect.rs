extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn security_wifi_threat_detect_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn security_wifi_threat_detect_exit() {
    // Cleanup logic for the module
}

pub struct WifiThreatDetector {
    threats: Vec<String>,
}

impl WifiThreatDetector {
    pub fn new() -> Self {
        WifiThreatDetector {
            threats: Vec::new(),
        }
    }

    pub fn add_threat(&mut self, threat: String) {
        self.threats.push(threat);
    }

    pub fn remove_threat(&mut self, threat: &str) {
        if let Some(index) = self.threats.iter().position(|t| t == threat) {
            self.threats.remove(index);
        }
    }

    pub fn is_threat_present(&self, threat: &str) -> bool {
        self.threats.contains(&String::from(threat))
    }

    pub fn list_threats(&self) -> Vec<String> {
        self.threats.clone()
    }

    pub fn clear_threats(&mut self) {
        self.threats.clear();
    }
}

#[no_mangle]
pub extern "C" fn security_wifi_threat_detect_add_threat(detector: *mut WifiThreatDetector, threat: *const u8, len: usize) {
    if let Some(detector) = unsafe { detector.as_mut() } {
        let threat_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(threat, len)) };
        detector.add_threat(String::from(threat_str));
    }
}

#[no_mangle]
pub extern "C" fn security_wifi_threat_detect_remove_threat(detector: *mut WifiThreatDetector, threat: *const u8, len: usize) {
    if let Some(detector) = unsafe { detector.as_mut() } {
        let threat_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(threat, len)) };
        detector.remove_threat(threat_str);
    }
}

#[no_mangle]
pub extern "C" fn security_wifi_threat_detect_is_threat_present(detector: *const WifiThreatDetector, threat: *const u8, len: usize) -> bool {
    if let Some(detector) = unsafe { detector.as_ref() } {
        let threat_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(threat, len)) };
        detector.is_threat_present(threat_str)
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn security_wifi_threat_detect_list_threats(detector: *const WifiThreatDetector) -> Vec<String> {
    if let Some(detector) = unsafe { detector.as_ref() } {
        detector.list_threats()
    } else {
        Vec::new()
    }
}

#[no_mangle]
pub extern "C" fn security_wifi_threat_detect_clear_threats(detector: *mut WifiThreatDetector) {
    if let Some(detector) = unsafe { detector.as_mut() } {
        detector.clear_threats();
    }
}
