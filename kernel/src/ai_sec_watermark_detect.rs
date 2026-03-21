extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn ai_sec_watermark_detect_init() -> i32 {
    0
}

pub extern "C" fn ai_sec_watermark_detect_exit() -> i32 {
    0
}

pub struct WatermarkDetector {
    watermarks: Vec<String>,
    detected: bool,
}

impl WatermarkDetector {
    pub fn new() -> Self {
        WatermarkDetector {
            watermarks: Vec::new(),
            detected: false,
        }
    }

    pub fn add_watermark(&mut self, watermark: String) {
        self.watermarks.push(watermark);
    }

    pub fn remove_watermark(&mut self, watermark: &str) -> bool {
        if let Some(index) = self.watermarks.iter().position(|w| w == watermark) {
            self.watermarks.remove(index);
            true
        } else {
            false
        }
    }

    pub fn detect(&mut self, data: &[u8]) -> bool {
        for watermark in &self.watermarks {
            if data.windows(watermark.len()).any(|window| window == watermark.as_bytes()) {
                self.detected = true;
                return true;
            }
        }
        false
    }

    pub fn reset_detection(&mut self) {
        self.detected = false;
    }

    pub fn is_detected(&self) -> bool {
        self.detected
    }
}
