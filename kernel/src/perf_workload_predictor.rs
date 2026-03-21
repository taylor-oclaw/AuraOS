extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct PerfWorkloadPredictor {
    data: Vec<u64>,
    predictions: Vec<f64>,
}

impl PerfWorkloadPredictor {
    pub fn new() -> Self {
        PerfWorkloadPredictor {
            data: Vec::new(),
            predictions: Vec::new(),
        }
    }

    pub fn add_data_point(&mut self, value: u64) {
        self.data.push(value);
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
        self.predictions.clear();
    }

    pub fn predict_next(&mut self) -> Option<f64> {
        if self.data.is_empty() {
            return None;
        }

        let sum: u64 = self.data.iter().sum();
        let avg = sum as f64 / self.data.len() as f64;

        // Simple prediction: next value is the average of current data
        self.predictions.push(avg);
        Some(avg)
    }

    pub fn get_predictions(&self) -> &Vec<f64> {
        &self.predictions
    }

    pub fn get_data_points(&self) -> &Vec<u64> {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perf_workload_predictor() {
        let mut predictor = PerfWorkloadPredictor::new();
        predictor.add_data_point(10);
        predictor.add_data_point(20);
        predictor.add_data_point(30);

        assert_eq!(predictor.get_data_points(), &[10, 20, 30]);

        let prediction = predictor.predict_next().unwrap();
        assert_eq!(prediction, 20.0);
        assert_eq!(predictor.get_predictions(), &[20.0]);

        predictor.clear_data();
        assert!(predictor.get_data_points().is_empty());
        assert!(predictor.get_predictions().is_empty());
    }
}
