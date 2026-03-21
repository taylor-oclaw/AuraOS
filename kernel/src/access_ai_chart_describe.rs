extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct AIChartDescriber {
    data: Vec<f64>,
    description: String,
}

impl AIChartDescriber {
    pub fn new(data: Vec<f64>) -> Self {
        AIChartDescriber {
            data,
            description: String::new(),
        }
    }

    pub fn analyze(&mut self) {
        if self.data.is_empty() {
            self.description = String::from("No data to analyze.");
            return;
        }

        let mean = self.calculate_mean();
        let median = self.calculate_median();
        let max = self.find_max();
        let min = self.find_min();
        let variance = self.calculate_variance(mean);

        self.description = String::from("info");
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    fn calculate_mean(&self) -> f64 {
        let sum: f64 = self.data.iter().sum();
        sum / self.data.len() as f64
    }

    fn calculate_median(&mut self) -> f64 {
        if self.data.is_empty() {
            return 0.0;
        }

        self.data.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mid = self.data.len() / 2;
        if self.data.len() % 2 == 0 {
            (self.data[mid - 1] + self.data[mid]) / 2.0
        } else {
            self.data[mid]
        }
    }

    fn find_max(&self) -> f64 {
        *self.data.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&0.0)
    }

    fn find_min(&self) -> f64 {
        *self.data.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&0.0)
    }

    fn calculate_variance(&self, mean: f64) -> f64 {
        let sum_of_squares: f64 = self.data.iter().map(|x| (x - mean).powi(2)).sum();
        sum_of_squares / self.data.len() as f64
    }
}
