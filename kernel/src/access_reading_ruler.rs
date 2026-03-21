extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct AccessReadingRuler {
    readings: Vec<String>,
}

impl AccessReadingRuler {
    pub fn new() -> Self {
        AccessReadingRuler {
            readings: Vec::new(),
        }
    }

    pub fn add_reading(&mut self, reading: String) {
        self.readings.push(reading);
    }

    pub fn get_readings(&self) -> &Vec<String> {
        &self.readings
    }

    pub fn clear_readings(&mut self) {
        self.readings.clear();
    }

    pub fn has_reading(&self, reading: &str) -> bool {
        self.readings.iter().any(|r| r == reading)
    }

    pub fn count_readings(&self) -> usize {
        self.readings.len()
    }
}
