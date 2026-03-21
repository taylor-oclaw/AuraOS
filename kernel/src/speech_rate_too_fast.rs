extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct SpeechRateMonitor {
    max_rate: u32,
    current_rate: u32,
    history: Vec<u32>,
}

impl SpeechRateMonitor {
    pub fn new(max_rate: u32) -> Self {
        SpeechRateMonitor {
            max_rate,
            current_rate: 0,
            history: Vec::new(),
        }
    }

    pub fn update_rate(&mut self, rate: u32) {
        if rate > self.max_rate {
            self.current_rate = self.max_rate;
        } else {
            self.current_rate = rate;
        }
        self.history.push(rate);
    }

    pub fn get_current_rate(&self) -> u32 {
        self.current_rate
    }

    pub fn is_too_fast(&self) -> bool {
        self.current_rate == self.max_rate
    }

    pub fn get_history(&self) -> &Vec<u32> {
        &self.history
    }

    pub fn reset(&mut self) {
        self.current_rate = 0;
        self.history.clear();
    }
}
