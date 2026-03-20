extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn sensor_hub_init() {
    // Initialization logic for the sensor hub module
}

pub extern "C" fn sensor_hub_exit() {
    // Cleanup logic for the sensor hub module
}

pub struct SensorHub {
    sensors: Vec<String>,
    data: Vec<f32>,
}

impl SensorHub {
    pub fn new() -> Self {
        SensorHub {
            sensors: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn add_sensor(&mut self, sensor_name: &str) {
        self.sensors.push(String::from(sensor_name));
        self.data.push(0.0); // Initialize with default value
    }

    pub fn remove_sensor(&mut self, sensor_name: &str) -> bool {
        if let Some(index) = self.sensors.iter().position(|s| s == sensor_name) {
            self.sensors.remove(index);
            self.data.remove(index);
            true
        } else {
            false
        }
    }

    pub fn update_sensor_data(&mut self, sensor_name: &str, value: f32) -> bool {
        if let Some(index) = self.sensors.iter().position(|s| s == sensor_name) {
            self.data[index] = value;
            true
        } else {
            false
        }
    }

    pub fn get_sensor_data(&self, sensor_name: &str) -> Option<f32> {
        self.sensors.iter().position(|s| s == sensor_name).map(|index| self.data[index])
    }

    pub fn list_sensors(&self) -> Vec<&String> {
        self.sensors.iter().collect()
    }
}
