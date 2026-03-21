extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct SleepWakePattern {
    sleep_times: Vec<u64>,
    wake_times: Vec<u64>,
}

impl SleepWakePattern {
    pub fn new() -> Self {
        SleepWakePattern {
            sleep_times: Vec::new(),
            wake_times: Vec::new(),
        }
    }

    pub fn record_sleep(&mut self, timestamp: u64) {
        self.sleep_times.push(timestamp);
    }

    pub fn record_wake(&mut self, timestamp: u64) {
        self.wake_times.push(timestamp);
    }

    pub fn get_sleep_count(&self) -> usize {
        self.sleep_times.len()
    }

    pub fn get_wake_count(&self) -> usize {
        self.wake_times.len()
    }

    pub fn calculate_average_sleep_duration(&self) -> Option<u64> {
        if self.sleep_times.is_empty() || self.wake_times.is_empty() {
            return None;
        }

        let mut total_duration = 0;
        for i in 0..self.sleep_times.len().min(self.wake_times.len()) {
            total_duration += self.wake_times[i] - self.sleep_times[i];
        }

        Some(total_duration / self.sleep_times.len() as u64)
    }
}
