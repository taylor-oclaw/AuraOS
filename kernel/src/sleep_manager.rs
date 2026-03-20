extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SleepManager {
    sleep_times: Vec<u64>,
    total_sleep_time: u64,
}

impl SleepManager {
    pub fn new() -> Self {
        SleepManager {
            sleep_times: Vec::new(),
            total_sleep_time: 0,
        }
    }

    pub fn add_sleep_time(&mut self, time: u64) {
        self.sleep_times.push(time);
        self.total_sleep_time += time;
    }

    pub fn get_total_sleep_time(&self) -> u64 {
        self.total_sleep_time
    }

    pub fn get_average_sleep_time(&self) -> Option<u64> {
        if self.sleep_times.is_empty() {
            None
        } else {
            Some(self.total_sleep_time / self.sleep_times.len() as u64)
        }
    }

    pub fn get_longest_sleep_time(&self) -> Option<u64> {
        self.sleep_times.iter().max().copied()
    }

    pub fn get_shortest_sleep_time(&self) -> Option<u64> {
        self.sleep_times.iter().min().copied()
    }
}
