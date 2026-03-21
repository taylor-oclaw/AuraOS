extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    println!("AI Security Model Theft Detection Module Loaded");
    0
}

pub struct AIThreatDetection {
    model_name: String,
    detection_threshold: f32,
    detected_threats: Vec<String>,
}

impl AIThreatDetection {
    pub fn new(model_name: &str, threshold: f32) -> Self {
        AIThreatDetection {
            model_name: String::from(model_name),
            detection_threshold: threshold,
            detected_threats: Vec::new(),
        }
    }

    pub fn update_model(&mut self, new_model_name: &str) {
        self.model_name = String::from(new_model_name);
    }

    pub fn set_detection_threshold(&mut self, threshold: f32) {
        self.detection_threshold = threshold;
    }

    pub fn detect_threat(&mut self, threat_level: f32, threat_description: &str) -> bool {
        if threat_level > self.detection_threshold {
            self.detected_threats.push(String::from(threat_description));
            true
        } else {
            false
        }
    }

    pub fn get_detected_threats(&self) -> Vec<String> {
        self.detected_threats.clone()
    }

    pub fn clear_detected_threats(&mut self) {
        self.detected_threats.clear();
    }
}
