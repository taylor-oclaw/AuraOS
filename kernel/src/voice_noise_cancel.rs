extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct VoiceNoiseCancel {
    samples: Vec<i16>,
    filter_coefficients: Vec<f32>,
}

impl VoiceNoiseCancel {
    pub fn new() -> Self {
        VoiceNoiseCancel {
            samples: Vec::new(),
            filter_coefficients: vec![0.5, -0.5], // Example coefficients
        }
    }

    pub fn add_sample(&mut self, sample: i16) {
        self.samples.push(sample);
    }

    pub fn get_samples(&self) -> &Vec<i16> {
        &self.samples
    }

    pub fn apply_filter(&mut self) {
        if self.samples.len() < 2 {
            return;
        }
        let mut filtered_samples = Vec::new();
        for i in 0..self.samples.len() - 1 {
            let filtered_sample = (self.filter_coefficients[0] * self.samples[i] as f32 +
                                 self.filter_coefficients[1] * self.samples[i + 1] as f32) as i16;
            filtered_samples.push(filtered_sample);
        }
        self.samples = filtered_samples;
    }

    pub fn set_filter_coefficients(&mut self, coefficients: Vec<f32>) {
        self.filter_coefficients = coefficients;
    }

    pub fn get_filter_coefficients(&self) -> &Vec<f32> {
        &self.filter_coefficients
    }
}
