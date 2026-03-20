extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct AntiAliasing {
    samples: Vec<f32>,
    width: usize,
    height: usize,
}

impl AntiAliasing {
    pub fn new(width: usize, height: usize) -> Self {
        let num_samples = width * height;
        AntiAliasing {
            samples: vec![0.0; num_samples],
            width,
            height,
        }
    }

    pub fn set_sample(&mut self, x: usize, y: usize, value: f32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.samples[index] = value;
        }
    }

    pub fn get_sample(&self, x: usize, y: usize) -> Option<f32> {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            Some(self.samples[index])
        } else {
            None
        }
    }

    pub fn average_samples(&self) -> f32 {
        let total: f32 = self.samples.iter().sum();
        total / (self.width * self.height) as f32
    }

    pub fn clear_samples(&mut self) {
        for sample in &mut self.samples {
            *sample = 0.0;
        }
    }

    pub fn apply_box_filter(&mut self, kernel_size: usize) {
        let half_kernel = kernel_size / 2;
        let mut new_samples = vec![0.0; self.samples.len()];

        for y in 0..self.height {
            for x in 0..self.width {
                let mut sum = 0.0;
                let mut count = 0;

                for ky in -half_kernel..=half_kernel {
                    for kx in -half_kernel..=half_kernel {
                        if let Some(value) = self.get_sample(x + kx, y + ky) {
                            sum += value;
                            count += 1;
                        }
                    }
                }

                if count > 0 {
                    new_samples[y * self.width + x] = sum / count as f32;
                }
            }
        }

        self.samples = new_samples;
    }
}
