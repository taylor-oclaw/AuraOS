extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct SpeechImpedimentCalibrator {
    data: Vec<u8>,
    calibration_factor: u32,
}

impl SpeechImpedimentCalibrator {
    pub fn new() -> Self {
        SpeechImpedimentCalibrator {
            data: Vec::new(),
            calibration_factor: 1,
        }
    }

    pub fn load_data(&mut self, data: &[u8]) {
        self.data.clear();
        self.data.extend_from_slice(data);
    }

    pub fn set_calibration_factor(&mut self, factor: u32) {
        if factor > 0 {
            self.calibration_factor = factor;
        }
    }

    pub fn get_calibration_factor(&self) -> u32 {
        self.calibration_factor
    }

    pub fn process_data(&self) -> Vec<u8> {
        let mut processed_data = Vec::with_capacity(self.data.len());
        for &byte in &self.data {
            processed_data.push((byte as u32 * self.calibration_factor % 256) as u8);
        }
        processed_data
    }

    pub fn analyze_impediment(&self) -> String {
        if self.data.is_empty() {
            return "No data to analyze".into();
        }

        let mut analysis = String::new();
        for (i, &byte) in self.data.iter().enumerate() {
            analysis.push_str(&format!("Byte {}: {}\n", i, byte));
        }
        analysis
    }
}
