extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut tracker = ResponseTimeTracker::new();
    tracker.record_response_time(10);
    tracker.record_response_time(20);
    tracker.record_response_time(30);

}

pub struct ResponseTimeTracker {
    times: Vec<u32>,
}

impl ResponseTimeTracker {
    pub fn new() -> Self {
        ResponseTimeTracker { times: Vec::new() }
    }

    pub fn record_response_time(&mut self, time: u32) {
        self.times.push(time);
    }

    pub fn average_response_time(&self) -> f32 {
        if self.times.is_empty() {
            0.0
        } else {
            let total: u32 = self.times.iter().sum();
            total as f32 / self.times.len() as f32
        }
    }

    pub fn max_response_time(&self) -> Option<u32> {
        self.times.iter().max().cloned()
    }

    pub fn min_response_time(&self) -> Option<u32> {
        self.times.iter().min().cloned()
    }

    pub fn total_responses(&self) -> usize {
        self.times.len()
    }
}
