extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn health_ambient_noise_monitor_init() {
    // Initialization logic for the module
}

pub extern "C" fn health_ambient_noise_monitor_exit() {
    // Cleanup logic for the module
}

pub struct NoiseMonitor {
    samples: Vec<u16>,
    threshold: u16,
}

impl NoiseMonitor {
    pub fn new(threshold: u16) -> Self {
        NoiseMonitor {
            samples: Vec::new(),
            threshold,
        }
    }

    pub fn add_sample(&mut self, sample: u16) {
        self.samples.push(sample);
    }

    pub fn get_average_noise_level(&self) -> Option<u16> {
        if self.samples.is_empty() {
            None
        } else {
            let sum: u32 = self.samples.iter().map(|&s| s as u32).sum();
            Some((sum / self.samples.len() as u32) as u16)
        }
    }

    pub fn is_noisy(&self) -> bool {
        if let Some(avg) = self.get_average_noise_level() {
            avg > self.threshold
        } else {
            false
        }
    }

    pub fn clear_samples(&mut self) {
        self.samples.clear();
    }

    pub fn get_threshold(&self) -> u16 {
        self.threshold
    }
}
