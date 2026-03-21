extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct LoadAvgCalc {
    load_avg: f64,
    samples: Vec<f64>,
}

impl LoadAvgCalc {
    pub fn new() -> Self {
        LoadAvgCalc {
            load_avg: 0.0,
            samples: Vec::new(),
        }
    }

    pub fn add_sample(&mut self, sample: f64) {
        self.samples.push(sample);
        if self.samples.len() > 10 {
            self.samples.remove(0);
        }
        self.update_load_avg();
    }

    pub fn get_load_avg(&self) -> f64 {
        self.load_avg
    }

    pub fn clear_samples(&mut self) {
        self.samples.clear();
        self.load_avg = 0.0;
    }

    pub fn num_samples(&self) -> usize {
        self.samples.len()
    }

    fn update_load_avg(&mut self) {
        if self.samples.is_empty() {
            return;
        }
        let sum: f64 = self.samples.iter().sum();
        self.load_avg = sum / self.samples.len() as f64;
    }
}
