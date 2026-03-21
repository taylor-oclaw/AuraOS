extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct VoiceCloneDetector {
    samples: Vec<f32>,
    threshold: f32,
}

impl VoiceCloneDetector {
    pub fn new(threshold: f32) -> Self {
        VoiceCloneDetector {
            samples: Vec::new(),
            threshold,
        }
    }

    pub fn add_sample(&mut self, sample: f32) {
        self.samples.push(sample);
    }

    pub fn get_samples_count(&self) -> usize {
        self.samples.len()
    }

    pub fn calculate_average(&self) -> Option<f32> {
        if self.samples.is_empty() {
            None
        } else {
            let sum: f32 = self.samples.iter().sum();
            Some(sum / self.samples.len() as f32)
        }
    }

    pub fn detect_clone(&self, sample: f32) -> bool {
        if let Some(avg) = self.calculate_average() {
            (sample - avg).abs() < self.threshold
        } else {
            false
        }
    }

    pub fn clear_samples(&mut self) {
        self.samples.clear();
    }
}
