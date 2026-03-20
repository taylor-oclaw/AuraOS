extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn proximity_sensor_init() {
    // Initialization logic for the proximity sensor module
}

pub extern "C" fn proximity_sensor_exit() {
    // Cleanup logic for the proximity sensor module
}

pub struct ProximitySensor {
    readings: Vec<u16>,
    threshold: u16,
}

impl ProximitySensor {
    pub fn new(threshold: u16) -> Self {
        ProximitySensor {
            readings: Vec::new(),
            threshold,
        }
    }

    pub fn add_reading(&mut self, reading: u16) {
        self.readings.push(reading);
    }

    pub fn get_readings(&self) -> &Vec<u16> {
        &self.readings
    }

    pub fn is_object_near(&self) -> bool {
        if let Some(&last_reading) = self.readings.last() {
            last_reading < self.threshold
        } else {
            false
        }
    }

    pub fn average_reading(&self) -> Option<u16> {
        if self.readings.is_empty() {
            None
        } else {
            let sum: u32 = self.readings.iter().map(|&r| r as u32).sum();
            Some((sum / self.readings.len() as u32) as u16)
        }
    }

    pub fn clear_readings(&mut self) {
        self.readings.clear();
    }
}
