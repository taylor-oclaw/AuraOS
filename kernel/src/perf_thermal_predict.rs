extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct PerfThermalPredict {
    temperature: i32,
    performance_level: u8,
    history: Vec<i32>,
}

impl PerfThermalPredict {
    pub fn new(initial_temp: i32) -> Self {
        PerfThermalPredict {
            temperature: initial_temp,
            performance_level: 5, // Assuming a scale from 1 to 10
            history: Vec::new(),
        }
    }

    pub fn update_temperature(&mut self, new_temp: i32) {
        self.history.push(self.temperature);
        self.temperature = new_temp;
        self.adjust_performance();
    }

    pub fn get_current_temperature(&self) -> i32 {
        self.temperature
    }

    pub fn get_performance_level(&self) -> u8 {
        self.performance_level
    }

    pub fn get_temperature_history(&self) -> &Vec<i32> {
        &self.history
    }

    fn adjust_performance(&mut self) {
        if self.temperature > 70 {
            // Reduce performance to prevent overheating
            self.performance_level = 3;
        } else if self.temperature < 60 {
            // Increase performance for better efficiency
            self.performance_level = 8;
        } else {
            // Maintain current performance level
            self.performance_level = 5;
        }
    }
}
