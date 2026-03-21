extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rel_response_time_expect_init() {
    // Initialization logic for the module
}

pub extern "C" fn rel_response_time_expect_exit() {
    // Cleanup logic for the module
}

pub struct ResponseTimeExpectation {
    expected_times: Vec<u64>,
    actual_times: Vec<u64>,
}

impl ResponseTimeExpectation {
    pub fn new(expected_times: Vec<u64>) -> Self {
        ResponseTimeExpectation {
            expected_times,
            actual_times: Vec::new(),
        }
    }

    pub fn add_actual_time(&mut self, time: u64) {
        self.actual_times.push(time);
    }

    pub fn get_expected_times(&self) -> &Vec<u64> {
        &self.expected_times
    }

    pub fn get_actual_times(&self) -> &Vec<u64> {
        &self.actual_times
    }

    pub fn calculate_average_difference(&self) -> Option<f64> {
        if self.expected_times.len() != self.actual_times.len() || self.expected_times.is_empty() {
            return None;
        }

        let mut total_diff: u64 = 0;
        for (expected, actual) in self.expected_times.iter().zip(self.actual_times.iter()) {
            total_diff += if expected > actual { expected - actual } else { actual - expected };
        }

        Some(total_diff as f64 / self.expected_times.len() as f64)
    }
}
