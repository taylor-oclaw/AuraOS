extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AccelerometerDriver {
    readings: Vec<[i32; 3]>,
    name: String,
}

impl AccelerometerDriver {
    pub fn new(name: &str) -> Self {
        AccelerometerDriver {
            readings: Vec::new(),
            name: String::from(name),
        }
    }

    pub fn add_reading(&mut self, x: i32, y: i32, z: i32) {
        self.readings.push([x, y, z]);
    }

    pub fn get_last_reading(&self) -> Option<[i32; 3]> {
        self.readings.last().cloned()
    }

    pub fn get_average_reading(&self) -> [f64; 3] {
        if self.readings.is_empty() {
            return [0.0, 0.0, 0.0];
        }
        let mut sum = [0i64; 3];
        for reading in &self.readings {
            sum[0] += reading[0] as i64;
            sum[1] += reading[1] as i64;
            sum[2] += reading[2] as i64;
        }
        let count = self.readings.len() as f64;
        [sum[0] as f64 / count, sum[1] as f64 / count, sum[2] as f64 / count]
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn clear_readings(&mut self) {
        self.readings.clear();
    }
}