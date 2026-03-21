extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct NotifyLearnResponseTime {
    responses: Vec<(u64, u64)>, // (timestamp, response_time)
}

impl NotifyLearnResponseTime {
    pub fn new() -> Self {
        NotifyLearnResponseTime {
            responses: Vec::new(),
        }
    }

    pub fn add_response(&mut self, timestamp: u64, response_time: u64) {
        self.responses.push((timestamp, response_time));
    }

    pub fn get_average_response_time(&self) -> Option<u64> {
        if self.responses.is_empty() {
            return None;
        }
        let total_time: u64 = self.responses.iter().map(|&(_, time)| time).sum();
        Some(total_time / self.responses.len() as u64)
    }

    pub fn get_max_response_time(&self) -> Option<u64> {
        self.responses.iter().map(|&(_, time)| time).max()
    }

    pub fn get_min_response_time(&self) -> Option<u64> {
        self.responses.iter().map(|&(_, time)| time).min()
    }

    pub fn clear_responses(&mut self) {
        self.responses.clear();
    }
}
