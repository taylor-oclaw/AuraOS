extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AmbientLightSensor {
    sensor_id: u32,
    readings: Vec<u16>,
    threshold: u16,
}

impl AmbientLightSensor {
    pub fn new(sensor_id: u32, initial_threshold: u16) -> Self {
        AmbientLightSensor {
            sensor_id,
            readings: Vec::new(),
            threshold: initial_threshold,
        }
    }

    pub fn add_reading(&mut self, reading: u16) {
        self.readings.push(reading);
    }

    pub fn get_latest_reading(&self) -> Option<u16> {
        self.readings.last().cloned()
    }

    pub fn get_average_reading(&self) -> Option<u16> {
        if self.readings.is_empty() {
            return None;
        }
        let sum: u32 = self.readings.iter().map(|&r| r as u32).sum();
        Some((sum / self.readings.len() as u32) as u16)
    }

    pub fn is_above_threshold(&self, reading: u16) -> bool {
        reading > self.threshold
    }

    pub fn set_threshold(&mut self, new_threshold: u16) {
        self.threshold = new_threshold;
    }
}
