extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let thermostat = SmartHomeThermostat::new(22.0, 18.0, 30.0);
    thermostat.set_temperature(24.5);
}

pub struct SmartHomeThermostat {
    current_temperature: f32,
    target_temperature: f32,
    min_temperature: f32,
    max_temperature: f32,
    history: Vec<f32>,
}

impl SmartHomeThermostat {
    pub fn new(initial_temp: f32, min_temp: f32, max_temp: f32) -> Self {
        let mut thermostat = SmartHomeThermostat {
            current_temperature: initial_temp,
            target_temperature: initial_temp,
            min_temperature: min_temp,
            max_temperature: max_temp,
            history: Vec::new(),
        };
        thermostat.history.push(initial_temp);
        thermostat
    }

    pub fn set_temperature(&mut self, temp: f32) {
        if temp >= self.min_temperature && temp <= self.max_temperature {
            self.current_temperature = temp;
            self.history.push(temp);
        }
    }

    pub fn get_current_temperature(&self) -> f32 {
        self.current_temperature
    }

    pub fn get_target_temperature(&self) -> f32 {
        self.target_temperature
    }

    pub fn is_heating(&self) -> bool {
        self.current_temperature < self.target_temperature
    }

    pub fn is_cooling(&self) -> bool {
        self.current_temperature > self.target_temperature
    }

    pub fn get_history(&self) -> &Vec<f32> {
        &self.history
    }
}
