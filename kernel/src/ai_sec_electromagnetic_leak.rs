extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct ElectromagneticLeakDetector {
    data: Vec<u8>,
    threshold: u8,
}

impl ElectromagneticLeakDetector {
    pub fn new(threshold: u8) -> Self {
        ElectromagneticLeakDetector {
            data: Vec::new(),
            threshold,
        }
    }

    pub fn add_sample(&mut self, sample: u8) {
        self.data.push(sample);
    }

    pub fn get_average(&self) -> Option<u8> {
        if self.data.is_empty() {
            None
        } else {
            let sum: u32 = self.data.iter().map(|&x| x as u32).sum();
            Some((sum / self.data.len() as u32) as u8)
        }
    }

    pub fn is_leak_detected(&self) -> bool {
        if let Some(avg) = self.get_average() {
            avg > self.threshold
        } else {
            false
        }
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn get_threshold(&self) -> u8 {
        self.threshold
    }
}
