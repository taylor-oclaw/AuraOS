extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct SpeechRateMonitor {
    min_rate: u32,
    max_rate: u32,
    current_rate: u32,
    history: Vec<u32>,
}

impl SpeechRateMonitor {
    pub fn new(min_rate: u32, max_rate: u32) -> Self {
        SpeechRateMonitor {
            min_rate,
            max_rate,
            current_rate: 0,
            history: Vec::new(),
        }
    }

    pub fn set_current_rate(&mut self, rate: u32) {
        if rate >= self.min_rate && rate <= self.max_rate {
            self.current_rate = rate;
            self.history.push(rate);
        } else {
            // Handle error or ignore invalid rate
        }
    }

    pub fn get_current_rate(&self) -> u32 {
        self.current_rate
    }

    pub fn is_too_slow(&self) -> bool {
        self.current_rate < self.min_rate
    }

    pub fn is_too_fast(&self) -> bool {
        self.current_rate > self.max_rate
    }

    pub fn get_history(&self) -> &Vec<u32> {
        &self.history
    }
}
