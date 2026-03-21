extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct PerfLatencyOptimizer {
    data: Vec<u64>,
    threshold: u64,
}

impl PerfLatencyOptimizer {
    pub fn new(threshold: u64) -> Self {
        PerfLatencyOptimizer {
            data: Vec::new(),
            threshold,
        }
    }

    pub fn add_latency(&mut self, latency: u64) {
        if latency > self.threshold {
            self.data.push(latency);
        }
    }

    pub fn get_high_latencies(&self) -> &[u64] {
        &self.data
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn average_latency(&self) -> Option<u64> {
        if self.data.is_empty() {
            None
        } else {
            let sum: u64 = self.data.iter().sum();
            Some(sum / self.data.len() as u64)
        }
    }

    pub fn max_latency(&self) -> Option<u64> {
        self.data.iter().max().copied()
    }
}
