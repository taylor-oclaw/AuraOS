extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct PerfResourceForecast {
    resources: Vec<String>,
    predictions: Vec<u32>,
}

impl PerfResourceForecast {
    pub fn new() -> Self {
        PerfResourceForecast {
            resources: Vec::new(),
            predictions: Vec::new(),
        }
    }

    pub fn add_resource(&mut self, resource: String) {
        self.resources.push(resource);
    }

    pub fn get_resources(&self) -> &Vec<String> {
        &self.resources
    }

    pub fn predict_load(&mut self, load: u32) {
        self.predictions.push(load);
    }

    pub fn get_predictions(&self) -> &Vec<u32> {
        &self.predictions
    }

    pub fn average_prediction(&self) -> Option<u32> {
        if self.predictions.is_empty() {
            None
        } else {
            let total: u32 = self.predictions.iter().sum();
            Some(total / self.predictions.len() as u32)
        }
    }
}
