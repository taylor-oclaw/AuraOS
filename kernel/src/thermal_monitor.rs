extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn thermal_monitor_init() {
    // Initialization logic for the thermal monitor module
}

#[no_mangle]
pub extern "C" fn thermal_monitor_exit() {
    // Cleanup logic for the thermal monitor module
}

pub struct ThermalMonitor {
    sensors: Vec<String>,
    temperatures: Vec<i32>,
}

impl ThermalMonitor {
    pub fn new(sensors: Vec<&str>) -> Self {
        let mut sensor_names = Vec::new();
        for name in sensors {
            sensor_names.push(String::from(name));
        }
        ThermalMonitor {
            sensors: sensor_names,
            temperatures: vec![0; sensors.len()],
        }
    }

    pub fn add_sensor(&mut self, sensor_name: &str) {
        self.sensors.push(String::from(sensor_name));
        self.temperatures.push(0);
    }

    pub fn remove_sensor(&mut self, index: usize) -> Option<String> {
        if index < self.sensors.len() {
            Some(self.sensors.remove(index))
        } else {
            None
        }
    }

    pub fn update_temperature(&mut self, index: usize, temperature: i32) {
        if index < self.temperatures.len() {
            self.temperatures[index] = temperature;
        }
    }

    pub fn get_temperatures(&self) -> Vec<i32> {
        self.temperatures.clone()
    }

    pub fn get_sensor_names(&self) -> Vec<String> {
        self.sensors.clone()
    }
}
